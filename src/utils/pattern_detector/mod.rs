mod structs;
mod mania;

use ::std::error::Error;
use reqwest::blocking;
use rosu_map;
use rosu_map::Beatmap;
use serde_json::Value;

pub fn download_file(url: &str) -> Result<String, Box<dyn Error>> {
    let response = blocking::get(url)?;
    if response.status().is_success() {
        Ok(response.text()?)
    } else {
        Err(format!("HTTP Error: {}", response.status()).into())
    }
}

pub(crate) fn get_patterns(path: &str) -> Result<Value, eyre::Report> {
    let map = rosu_map::from_path::<Beatmap>(&path).unwrap();
    if (map.mode == rosu_map::section::general::GameMode::Mania) {
        let result_json = mania::transformers(map);
        Ok(result_json)
    }
    else{
        Err(eyre::eyre!("Mode de jeu non supporté : seul Mania est pris en charge"))
    }
}
