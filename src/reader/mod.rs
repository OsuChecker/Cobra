use std::env::Args;
use eyre::{Report, Result};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use rosu_mem::error::ProcessError;
use rosu_mem::process::{Process, ProcessTraits};
use slint::{ComponentHandle, SharedString, Weak};
use tokio::sync::Mutex;
use crate::{AppState, LoginPage, MapData};
use crate::reader::reader_beatmap::{get_beatmap, get_beatmap_md5};
use crate::reader::reader_common::{get_status};
use crate::reader::reader_gameplay::{get_mods, get_retries};
use crate::reader::structs::{GameState, State, StaticAddresses};

mod reader_gameplay;
mod structs;
mod reader_resultscreen;
mod reader_common;
mod reader_beatmap;

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
        if SharedString::from(get_beatmap_md5(p, state)) != last_map.md5 {
            println!("New map");
            let map = get_beatmap(p, state);
            let weak_clone = weak.clone();
            let map_clone = map.clone();

                slint::spawn_local(async move  {
                    if let Some(handle) = weak_clone.upgrade() {
                        println!("Updating map: {} - {}", map_clone.song, map_clone.author);
                        handle.global::<AppState>().set_map(map_clone);
                    }
                });


            last_map = map.clone();
        }

        thread::sleep(Duration::from_millis(300));
    }
}



pub fn controlla(weak: Weak<LoginPage>) -> Result<()> {

    let interval : Duration = Duration::from_millis(300);
    let mut state = State {
        addresses: StaticAddresses::default(),
    };

    if interval != Duration::from_millis(300) {
        println!("Using non default interval: {}", interval.as_millis());
    }

    'init_loop:
        loop {
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
            if let Err(e) = waiting_for_play(&p, &mut state,weak_clone) {
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
    let mut last_retries =0;
    let mode_list = get_mods(p,state);
    while ( GameState::from(get_status(p,state)) == GameState::Playing)
    {
        cur_time = reader_gameplay::get_ig_time(p,state);
        if (cur_time-last_time < 20 && cur_time > 0 && last_time > 0 && last_paused !=cur_time)
        {
            last_paused = cur_time;
        }
        last_time = cur_time;
        let status = get_status(p, state);
        let md5 = get_beatmap_md5(p, state);
        if (last_retries<get_retries(p,state)){
            return false
        }
    }
    true
}
