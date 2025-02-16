use reqwest::blocking;
use serde::Serialize;
use std::error::Error;
use std::fs;
use eyre::WrapErr;

#[derive(Debug, Serialize)]
pub struct KeyValue {
    pub key: i32,
    pub value: f64,
}

pub fn load_file(file_path: &str) -> String {
    fs::read_to_string(file_path).unwrap()
}

pub fn read_note(line: &str) -> eyre::Result<i32> {
    let mut parts = line.split(',');
    if let (Some(_), Some(_), Some(hit_object)) = (parts.next(), parts.next(), parts.next()) {
        hit_object
            .parse::<i32>()
            .wrap_err_with(|| format!("Erreur lors du parsing de l'objet ({hit_object})"))
    } else {
        Err(eyre::eyre!("Erreur 17001: Fichier .osu invalide (Impossible de lire hitObject)"))
    }
}


pub fn parse_hit_objects(curl_content: &str) -> eyre::Result<Vec<i32>> {
    let mut hit_objects = false;
    curl_content
        .lines()
        .filter_map(|line| {
            if !hit_objects {
                if line == "[HitObjects]" {
                    hit_objects = true;
                }
                None
            } else {
                match read_note(line) {
                    Ok(note) => Some(Ok(note)),
                    Err(e) => Some(Err(eyre::eyre!("Erreur lors de la lecture de la note: {}", e)))
                }
            }
        })
        .collect::<eyre::Result<Vec<i32>>>()
}

pub fn calculate_average_notes_per_second(timings: &[i32], frequency: f64) -> eyre::Result<Vec<KeyValue>> {
    if timings.is_empty() {
        return Err(eyre::eyre!("Le vecteur de timings est vide"));
    }

    let start_time = *timings.first().ok_or_else(|| eyre::eyre!("Impossible d'obtenir le premier timing"))?;
    let end_time = *timings.last().ok_or_else(|| eyre::eyre!("Impossible d'obtenir le dernier timing"))?;
    let t_duration: i32 = end_time - start_time;
    let interval_ms: f64 = (t_duration as f64 * frequency) / 100.0_f64;

    if interval_ms <= 0.0 {
        return Err(eyre::eyre!("Durée invalide ou fréquence trop élevée (interval_ms: {})", interval_ms));
    }

    let mut result = Vec::new();
    let mut interval_start = start_time;
    let mut current_note_index: usize = 0;

    while interval_start < end_time {
        let interval_end = interval_start + interval_ms as i32;
        let interval_end_index = timings.partition_point(|&x| x < interval_end);
        let note_count = interval_end_index - current_note_index;

        let interval_seconds = interval_ms / 1000.0_f64;
        let nps = note_count as f64 / interval_seconds;

        result.push(KeyValue {
            key: interval_start,
            value: nps,
        });

        interval_start = interval_end;
        current_note_index = interval_end_index;
    }

    Ok(result)
}

pub fn get_nps(url: &str, frequency: f64) -> eyre::Result<Vec<KeyValue>> {
    let curl_content = load_file(url);

    let parsed_hit_objects = parse_hit_objects(&curl_content)
        .wrap_err_with(|| "Erreur lors de l'analyse du fichier .osu")?;

    let nps_result = calculate_average_notes_per_second(&parsed_hit_objects, frequency)
        .wrap_err_with(|| "Erreur lors du calcul des NPS")?;


    Ok(nps_result)
}

