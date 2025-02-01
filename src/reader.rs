use crate::structs::{Arm, InnerValues, OutputValues};

use crate::reading_loop::process_reading_loop;
use crate::structs::{State, StaticAddresses};

use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use clap::Parser;
use druid::platform_menus::mac::file::print;
use rosu_mem::{
    error::ProcessError,
    process::{Process, ProcessTraits},
};

use eyre::{Report, Result};




#[derive(Parser, Debug)]
pub struct Args {
    /// Path to osu! folder
    #[arg(short, long, env)]
    osu_path: Option<PathBuf>,

    /// Interval between updates in ms
    #[clap(default_value = "200")]
    #[arg(short, long, value_parser=parse_interval_ms)]
    interval: std::time::Duration,

    /// Amount of seconds waiting after critical error happened
    /// before running again
    #[clap(default_value = "3")]
    #[arg(short, long, value_parser=parse_interval_secs)]
    error_interval: std::time::Duration,
}

fn parse_interval_ms(arg: &str) -> Result<std::time::Duration, std::num::ParseIntError> {
    let ms = arg.parse()?;
    Ok(std::time::Duration::from_millis(ms))
}

fn parse_interval_secs(arg: &str) -> Result<std::time::Duration, std::num::ParseIntError> {
    let secs = arg.parse()?;
    Ok(std::time::Duration::from_secs(secs))
}

pub async fn controlla() -> Result<()> {
    let _client = tracy_client::Client::start();

    let args = Args::parse();
    let output_values = Arc::new(Mutex::new(OutputValues::default()));
    let inner_values = InnerValues::default();

    let mut state = State {
        addresses: StaticAddresses::default(),
        ivalues: inner_values,
        values: output_values,
    };

    if args.interval != Duration::from_millis(300) {
        println!("Using non default interval: {}", args.interval.as_millis());
    }

    'init_loop: loop {
        let p = match Process::initialize("osu!.exe") {
            Ok(p) => p,
            Err(e) => {
                println!("{:?}", Report::new(e));
                thread::sleep(args.error_interval);
                continue 'init_loop;
            }
        };

        let mut values = state.values.lock().unwrap();
        // OSU_PATH cli argument if provided should
        // overwrite auto detected path
        // else use auto detected path
        match args.osu_path {
            Some(ref v) => {
                println!("Using provided osu! folder path");
                values.osu_path.clone_from(v);
            }
            None => {
                println!("Using auto-detected osu! folder path");
                if let Some(ref dir) = p.executable_dir {
                    values.osu_path.clone_from(dir);
                } else {
                    return Err(Report::msg(
                        "Can't auto-detect osu! folder path \
                         nor any was provided through command \
                         line argument",
                    ));
                }
            }
        }

        // Checking if path exists
        if !values.osu_path.exists() {
            println!(
                "Provided osu path doesn't exists!\n Path: {}",
                &values.osu_path.to_str().unwrap()
            );

            return Err(Report::msg(
                "Can't auto-detect osu! folder path \
                 nor any was provided through command \
                 line argument",
            ));
        };

        drop(values);

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
            if let Err(e) = crate::reader2::init_reading::waiting_for_play(&p, &mut state) {
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
