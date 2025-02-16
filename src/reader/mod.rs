use crate::reader::reader_beatmap::{get_audio_path, get_beatmap, get_beatmap_md5, get_beatmap_path, get_cover_path};
use crate::reader::reader_common::get_status;
use crate::reader::reader_gameplay::{get_mods, get_retries};
use crate::reader::structs::{GameState, State, StaticAddresses};
use crate::utils::nps::get_nps;
use crate::utils::pattern_detector::get_patterns;
use crate::{AppState, LoginPage, MapData, MapSetState};
use eyre::{Report, Result};
use rosu_mem::error::ProcessError;
use rosu_mem::process::{Process, ProcessTraits};
use serde_json::Value;
use slint::{ComponentHandle, Image, ModelRc, SharedString, VecModel, Weak};
use std::env::Args;
use std::os::windows::raw::HANDLE;
use std::path::Path;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use futures_util::task::SpawnExt;
use tokio::sync::Mutex;
use rayon::prelude::*;

mod reader_beatmap;
mod reader_common;
mod reader_gameplay;
mod reader_resultscreen;
mod structs;

fn analyze_patterns(json_value: &Value) -> Vec<SharedString> {
    let mut patterns = Vec::new();

    if let Some(tertiary) = json_value
        .get("TertiaryPattern")
        .and_then(|v| v.as_object())
    {
        let mut values: Vec<(String, f64)> = tertiary
            .iter()
            .filter_map(|(key, val)| val.as_f64().map(|v| (key.clone(), v)))
            .filter(|(_k, v)| *v > 0.0)
            .collect();

        values.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        if !values.is_empty() {
            let max_value = values[0].1;
            patterns = values
                .into_iter()
                .filter(|(_k, v)| *v >= (max_value * 0.8))
                .map(|(k, v)| format!("{}", k).into())
                .collect();
        }
    }

    patterns
}

pub fn waiting_for_play(p: &Process, state: &mut State, weak: Weak<LoginPage>) -> eyre::Result<()> {
    let mut last_map: MapData = MapData {
        song: "".into(),
        author: "".into(),
        creator: "".into(),
        cover: slint::Image::default(),
        link: "".into(),
        difficulties: "".into(),
        download_progress: 0.0,
        md5: "".into(),
    };

    loop {
        if GameState::from(get_status(p, state)) == GameState::Playing {
            return Ok(());
        }

        if SharedString::from(get_beatmap_md5(p, state)?) != last_map.md5 {
            println!("New map");
            let map = get_beatmap(p, state)?;
            let map_to_move = map.clone();
            last_map = map.clone();
            let handle = weak.clone();
            let path = get_beatmap_path(
                p,
                state
            )?;
            println!("{}", path);
            let img = get_cover_path(p, state)?;
            let audio = get_audio_path(p,state)?;

            std::thread::spawn(move || {
                let song = map_to_move.song.clone();
                let author = map_to_move.author.clone();
                let creator = map_to_move.creator.clone();
                let link = map_to_move.link.clone();
                let difficulties = map_to_move.difficulties.clone();
                let progress = map_to_move.download_progress;
                let md5 = map_to_move.md5.clone();
                let (calc_pp, (b, patterns)) = rayon::join(
                    || calc_pp(&path),
                    || rayon::join(
                        || get_nps(&path, 1.0).unwrap(),
                        || {
                            let patterns = get_patterns(&path).unwrap();
                            analyze_patterns(&patterns)
                        }
                    )
                );


                let values : Vec<f32> = b.iter().map(|kv| kv.value as f32).collect();
                let (avg, max) = {
                    let values: Vec<f32> = b.par_iter()
                        .map(|kv| kv.value as f32)
                        .collect();

                    let (sum, max) = values.par_iter()
                        .fold(
                            || (0.0f32, f32::NEG_INFINITY),
                            |(sum, max), &value| (sum + value, max.max(value))
                        )
                        .reduce(
                            || (0.0f32, f32::NEG_INFINITY),
                            |(sum1, max1), (sum2, max2)| (sum1 + sum2, max1.max(max2))
                        );

                    (sum / values.len() as f32, max)
                };

                handle.upgrade_in_event_loop(move |handle| {
                    let img = Image::load_from_path(Path::new(&img)).unwrap_or_else(|_| {
                        Image::default()
                    });
                    let map_data = MapData {
                        song,
                        author,
                        creator,
                        cover: img,
                        link,
                        difficulties,
                        download_progress: progress,
                        md5,
                    };
                    handle.global::<AppState>().set_map(map_data);
                    let model_data = ModelRc::new(VecModel::from(values));
                    handle.global::<AppState>().set_graph_data(model_data);
                    handle.global::<AppState>().set_avg_nps(avg);
                    handle.global::<AppState>().set_max_value(max);
                    handle.global::<AppState>().set_pp_text1(SharedString::from(format!("95%: {}", calc_pp[0])));
                    handle.global::<AppState>().set_pp_text2(SharedString::from(format!("98%: {}", calc_pp[1])));
                    handle.global::<AppState>().set_pp_text3(SharedString::from(format!("99%: {}", calc_pp[2])));
                    handle.global::<AppState>().set_pp_text4(SharedString::from(format!("100%: {}", calc_pp[3])));
                    let patterns = ModelRc::new(VecModel::from(patterns));
                    handle.global::<AppState>().set_patterns(patterns);
                    handle.global::<AppState>().set_audio_path(SharedString::from(audio));
                    handle.global::<AppState>().set_osu_path(SharedString::from(path));
                })
                    .expect("Échec de la mise à jour de l'interface");
            });



        }

        thread::sleep(Duration::from_millis(300));
    }
}

fn calc_pp(path: &String) -> Vec<i32> {
    let mut vec: Vec<i32> = vec![];
    let map = rosu_pp::Beatmap::from_path(path).unwrap();
    let diff_attrs = rosu_pp::Difficulty::new().mods(0).calculate(&map);
    let stars = diff_attrs.stars();

    let perf_attrs = rosu_pp::Performance::new(diff_attrs)
        .accuracy(95.0)
        .calculate();
    vec.push(perf_attrs.pp() as i32);

    let perf_attrs = perf_attrs.performance().accuracy(98.0).calculate();
    vec.push(perf_attrs.pp() as i32);

    let perf_attrs = perf_attrs.performance().accuracy(99.0).calculate();
    vec.push(perf_attrs.pp() as i32);

    vec.push(perf_attrs.performance().accuracy(100.0).calculate().pp() as i32);

    vec
}

pub fn controlla(weak: Weak<LoginPage>) -> Result<()> {
    let interval: Duration = Duration::from_millis(300);
    let mut state = State {
        addresses: StaticAddresses::default(),
    };

    if interval != Duration::from_millis(300) {
        println!("Using non default interval: {}", interval.as_millis());
    }

    'init_loop: loop {
        let p = match Process::initialize("osu!.exe") {
            Ok(p) => p,
            Err(e) => {
                continue 'init_loop;
            }
        };

        println!("Reading static signatures...");
        match StaticAddresses::new(&p) {
            Ok(v) => state.addresses = v,
            Err(e) => match e.downcast_ref::<ProcessError>() {
                Some(&ProcessError::ProcessNotFound) => {
                    continue 'init_loop;
                }
                #[cfg(target_os = "windows")]
                Some(&ProcessError::OsError { .. }) => {
                    println!("{:?}", e);
                    continue 'init_loop;
                }
                Some(_) | None => {
                    println!("{:?}", e);
                    continue 'init_loop;
                }
            },
        };

        println!("Starting reading loop");
        'main_loop: loop {
            std::thread::sleep(Duration::from_millis(1000));
            println!("Waiting For Play");
            let weak_clone = weak.clone();
            if let Err(e) = waiting_for_play(&p, &mut state, weak_clone) {
                match e.downcast_ref::<ProcessError>() {
                    Some(&ProcessError::ProcessNotFound) => {
                        continue 'init_loop;
                    }
                    #[cfg(target_os = "windows")]
                    Some(&ProcessError::OsError { .. }) => {
                        println!("{:?}", e);
                        continue 'init_loop;
                    }
                    Some(_) | None => {
                        println!("{:?}", e);
                        continue 'main_loop;
                    }
                }
            }
            println!("Playing");
            std::thread::sleep(Duration::from_millis(700));
            let a = playing(&p, &mut state);

            // let a = wait_result_screen(&p, &mut state) && a;
        }
    }
}

pub(crate) fn playing(p: &Process, state: &mut State) -> bool {
    println!("Playing");
    let mut cur_time = 0;
    let mut last_time = 0;
    let mut last_paused = 0;
    let mut last_retries = 0;
    let mode_list = get_mods(p, state);
    while (GameState::from(get_status(p, state)) == GameState::Playing) {
        cur_time = reader_gameplay::get_ig_time(p, state);
        if (cur_time - last_time < 20 && cur_time > 0 && last_time > 0 && last_paused != cur_time) {
            last_paused = cur_time;
        }
        last_time = cur_time;
        let status = get_status(p, state);
        let md5 = get_beatmap_md5(p, state);
        if (last_retries < get_retries(p, state)) {
            return false;
        }
    }
    true
}
