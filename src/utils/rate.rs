use rubato::{Resampler, SincFixedIn, SincInterpolationParameters, SincInterpolationType};
use std::error::Error;
use std::fs;
use std::io::Write;
use hound;
use symphonia::core::audio::AudioBufferRef;
use symphonia::core::audio::Signal;
use vorbis_encoder::Encoder as VorbisEncoder;
use rosu_map;

use hound::WavReader;
use rosu_map::Beatmap;
use rosu_map::section::hit_objects::HitObjectKind;
use rosu_map::section::timing_points::ControlPoints;

pub fn change_audio_speed_wav(input_path: &str, output_path: &str, speed: f32) -> eyre::Result<()> {
    let file = std::fs::File::open(input_path)?;
    let media_source = symphonia::core::io::MediaSourceStream::new(Box::new(file), Default::default());

    let mut hint = symphonia::core::probe::Hint::new();
    if input_path.ends_with(".mp3") {
        hint.with_extension("mp3");
    } else if input_path.ends_with(".wav") {
        hint.with_extension("wav");
    }

    let probe = symphonia::default::get_probe()
        .format(&hint, media_source, &Default::default(), &Default::default())?;

    let mut format = probe.format;
    let track = format.default_track().unwrap();
    let mut decoder = symphonia::default::get_codecs()
        .make(&track.codec_params, &Default::default())?;

    let sample_rate = track.codec_params.sample_rate.unwrap_or(44100);
    let channels = track.codec_params.channels.unwrap().count();

    // On garde un sample rate de qualité
    let output_sample_rate = 44100;

    let mut resampler = SincFixedIn::<f32>::new(
        (output_sample_rate as f64 / sample_rate as f64) * (1.0 / speed as f64),
        1.0,
        SincInterpolationParameters {
            sinc_len: 256,
            f_cutoff: 0.95,
            interpolation: rubato::SincInterpolationType::Cubic, // Meilleure qualité
            oversampling_factor: 256, // Retour à une meilleure qualité
            window: rubato::WindowFunction::BlackmanHarris2
        },
        1152,
        channels  // On garde le stéréo
    )?;

    let mut output_file = hound::WavWriter::create(
        output_path,
        hound::WavSpec {
            channels: channels as u16,
            sample_rate: output_sample_rate,
            bits_per_sample: 16,  // 16 bits est suffisant pour une bonne qualité
            sample_format: hound::SampleFormat::Int,
        }
    )?;

    let mut input_buffer = vec![Vec::new(); channels];
    let mut output_buffer = vec![Vec::new(); channels];
    let mut accumulated_samples: Vec<Vec<f32>> = vec![Vec::new(); channels];

    while let Ok(packet) = format.next_packet() {
        let decoded = decoder.decode(&packet)?;
        let frames = decoded.frames();

        match &decoded {
            AudioBufferRef::F32(buf) => {
                for frame in 0..frames {
                    for ch in 0..channels {
                        accumulated_samples[ch].push(*buf.chan(ch).get(frame).unwrap_or(&0.0));
                    }
                }
            },
            AudioBufferRef::S16(buf) => {
                for frame in 0..frames {
                    for ch in 0..channels {
                        accumulated_samples[ch].push(
                            *buf.chan(ch).get(frame).unwrap_or(&0) as f32 / 32768.0
                        );
                    }
                }
            },
            _ => return Err(eyre::eyre!("Format audio non supporté")),
        }

        while accumulated_samples[0].len() >= 1152 {
            for ch in 0..channels {
                input_buffer[ch].clear();
                input_buffer[ch].extend_from_slice(&accumulated_samples[ch][..1152]);
                accumulated_samples[ch].drain(..1152);
            }

            output_buffer = resampler.process(&input_buffer, None)?;

            for frame in 0..output_buffer[0].len() {
                for ch in 0..channels {
                    let sample = output_buffer[ch][frame];
                    // Légère compression dynamique pour éviter la distorsion
                    let compressed = if sample > 0.0 {
                        (sample * 0.95).min(0.95)
                    } else {
                        (sample * 0.95).max(-0.95)
                    };
                    let sample_i16 = (compressed * 32767.0) as i16;
                    output_file.write_sample(sample_i16)?;
                }
            }
        }
    }

    if !accumulated_samples[0].is_empty() {
        for ch in 0..channels {
            input_buffer[ch].clear();
            input_buffer[ch].extend_from_slice(&accumulated_samples[ch]);
            input_buffer[ch].resize(1152, 0.0);
        }

        if let Ok(final_output) = resampler.process(&input_buffer, None) {
            for frame in 0..final_output[0].len() {
                for ch in 0..channels {
                    let sample = final_output[ch][frame];
                    let compressed = if sample > 0.0 {
                        (sample * 0.95).min(0.95)
                    } else {
                        (sample * 0.95).max(-0.95)
                    };
                    let sample_i16 = (compressed * 32767.0) as i16;
                    output_file.write_sample(sample_i16)?;
                }
            }
        }
    }

    output_file.finalize()?;
    Ok(())
}

pub fn change_audio_speed(input_path: &str, output_path: &str, speed: f32) -> eyre::Result<()> {
    let temp_wav = format!("{}.temp.wav", output_path);

    change_audio_speed_wav(input_path, &temp_wav, speed)?;

    // Conversion en Ogg
    convert_wav_to_ogg(&temp_wav, output_path)?;

    fs::remove_file(&temp_wav)?;

    Ok(())
}
pub fn convert_wav_to_ogg(input_wav: &str, output_ogg: &str) -> eyre::Result<()> {
    let mut reader = hound::WavReader::open(input_wav)?;
    let spec = reader.spec();

    let mut encoder = VorbisEncoder::new(
        spec.channels as u32,
        spec.sample_rate as u64,
        0.5
    ).map_err(|e| eyre::eyre!("Erreur lors de la création de l'encodeur Vorbis: {}", e))?;

    let samples: Vec<i16> = reader.samples::<i16>()
        .map(|s| s.unwrap_or(0))
        .collect();

    let mut output_file = std::fs::File::create(output_ogg)?;

    let encoded_data = encoder.encode(&samples)
        .map_err(|e| eyre::eyre!("Erreur lors de l'encodage: {}", e))?;
    output_file.write_all(&encoded_data)?;

    let final_data = encoder.flush()
        .map_err(|e| eyre::eyre!("Erreur lors de la finalisation: {}", e))?;
    output_file.write_all(&final_data)?;

    Ok(())
}

pub fn change_osu_speed(input_path: &str, rate: f32, audio_path: &str) {
    let mut map: Beatmap = rosu_map::from_path(input_path).unwrap();

    let multiplier: f64 = 1.0f64 / rate as f64;
    map.audio_file = audio_path.to_string();
    map.version = format!("{} {:.2}x", map.version, rate);
    for point in map.control_points.timing_points.iter_mut() {
        point.time = (point.time as f64 * multiplier) as f64;
        point.beat_len = (point.beat_len as f64 * multiplier) as f64;
    }
    for point in map.control_points.difficulty_points.iter_mut() {
        point.time = (point.time as f64 * multiplier) as f64;
    }
    for point in map.control_points.effect_points.iter_mut() {
        point.time = (point.time as f64 * multiplier) as f64;
    }
    for point in map.control_points.sample_points.iter_mut() {
        point.time = (point.time as f64 * multiplier) as f64;
    }

    for hit_object in map.hit_objects.iter_mut() {
        hit_object.start_time = hit_object.start_time * multiplier;
        if let HitObjectKind::Hold(hold) = &mut hit_object.kind {
            hold.duration = hold.duration * multiplier;
        }
    }
    let path = std::path::Path::new(input_path);
    let stem = path.file_stem().unwrap().to_str().unwrap();
    let parent = path.parent().unwrap();
    let b = format!("{}/{}_{}x.osu",
            parent.display(),
            stem,
            rate
    );


    map.encode_to_path(b).unwrap();

}