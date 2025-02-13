use std::env::Args;
use eyre::{Report, Result};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use rosu_mem::error::ProcessError;
use rosu_mem::process::{Process, ProcessTraits};
use tokio::sync::Mutex;
use crate::reader::structs::{State, StaticAddresses};

mod reader_gameplay;
mod structs;
mod reader_resultscreen;

// Depuis l'ancienne version de Cobra
/*
pub fn waiting_for_play(p: &Process, state: &mut State) -> eyre::Result<()> {
    loop {
        if crate::structs::GameState::from(get_status(p, state)) == crate::structs::GameState::Playing {
            return Result::Ok(())
        }
    }

}

pub async fn controlla() -> Result<()> {

    let args;

    let mut state = State {
        addresses: StaticAddresses::default(),
    };

    if args.interval != Duration::from_millis(300) {
        println!("Using non default interval: {}", args.interval.as_millis());
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
                    thread::sleep(args.error_interval);
                    continue 'init_loop;
                }
                #[cfg(target_os = "windows")]
                Some(&ProcessError::OsError { .. }) => {
                    println!("{:?}", e);
                    thread::sleep(args.error_interval);
                    continue 'init_loop;
                }
                Some(_) | None => {
                    println!("{:?}", e);
                    thread::sleep(args.error_interval);
                    continue 'init_loop;
                }
            },
        };

        println!("Starting reading loop");
        'main_loop: loop {
            std::thread::sleep(Duration::from_millis(1000));
            println!("Waiting For Play");
            if let Err(e) = waiting_for_play(&p, &mut state) {
                match e.downcast_ref::<ProcessError>() {
                    Some(&ProcessError::ProcessNotFound) => {
                        thread::sleep(args.error_interval);
                        continue 'init_loop;
                    }
                    #[cfg(target_os = "windows")]
                    Some(&ProcessError::OsError { .. }) => {
                        println!("{:?}", e);
                        thread::sleep(args.error_interval);
                        continue 'init_loop;
                    }
                    Some(_) | None => {
                        println!("{:?}", e);
                        thread::sleep(args.error_interval);
                        continue 'main_loop;
                    }
                }
            }
            println!("Playing");
            std::thread::sleep(Duration::from_millis(700));
            let a = crate::reader2::init_reading::playing(&p, &mut state);

            let a = crate::reader2::init_reading::wait_result_screen(&p, &mut state) && a;
            std::thread::sleep(Duration::from_millis(500));

            if (a){
                crate::reader2::init_reading::send_score(&p, &mut state);
            }

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
    sendArq(p,state);
    while ( crate::structs::GameState::from(get_status(p,state)) == crate::structs::GameState::Playing) {
        cur_time = crate::reader2::reader_gameplay::get_ig_time(p,state);
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
*/