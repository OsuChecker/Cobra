use std::time::Duration;
use crate::structs::{OutputValues, State};
use rosu_mem::process::{Process, ProcessTraits};
use crate::global::get_token;
use crate::reader2::reader_gameplay::{get_mods, get_retries};
use crate::reader2::reader_resultscreen;
use crate::reader2::reader_resultscreen::{get_result_hit_100, get_result_hit_50, get_result_hit_geki, get_result_hit_katu, get_result_hit_miss};

pub fn waiting_for_play(p: &Process, state: &mut State) -> eyre::Result<()> {
    loop {
        let values = state.values.clone();
        let mut values = values.lock().unwrap();
        if crate::structs::GameState::from(get_status(p, state)) == crate::structs::GameState::Playing {
            return Result::Ok(())
        }
    }

}
pub(crate) fn wait_result_screen(p: &Process, state: &mut State) -> bool{

    while (crate::structs::GameState::from(get_status(p, state)) == crate::structs::GameState::Playing) {
        std::thread::sleep(Duration::from_millis(10));

    }
    if crate::structs::GameState::from(get_status(p, state)) == crate::structs::GameState::ResultScreen {
            return true
    }
    return false

}
fn get_status(p: &Process, state: &mut State) -> u32 {
    let status_ptr = p.read_i32(state.addresses.status - 0x4).unwrap();
    p.read_u32(status_ptr).unwrap()
}
fn get_beatmap_md5(p: &Process, state: &mut State) -> String {
    let ruleset_addr = p.read_i32(state.addresses.base - 0xC).unwrap();
    let ruleset_addr = p.read_i32(ruleset_addr ).unwrap();
    p.read_string(ruleset_addr + 0x6c).unwrap()
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

pub fn sendArq(p: &Process, state: &mut State){
    println!("Sending Beatmap");
    let token = get_token().unwrap();
    let md5 = get_beatmap_md5(p,state);
    let body = format!(r#"{{"token": "{token}","map": {{"checkSum": "{md5}"}}}}"#);
    let url = "https://osef.me/api/request"; // Remplace par ton URL

    let result = crate::utils::request("POST", url, Some(body));

    if (result.is_ok()) {
        println!("{:?}", "Succesfully added beatmap")
    } else {
        println!("{:?}", result.err())
    }
}
fn mods_to_string(mod_bits: u32) -> String {
    let mut mod_list = Vec::new();

    for (bit, name) in crate::structs::MODS.iter() {
        if mod_bits & bit != 0 {
            if *name == "NC" {
                continue; // Ignore "NC"
            }
            mod_list.push(*name); // Ajoute le mod correspondant si le bit est activé
        }

    }

    mod_list.join(",")

}
pub fn send_score(p : &Process, state: &mut State) -> bool{

    let body = format!(
        r#"{{"token": "{token}", "map": {{"checkSum": "{md5}"}}, "score": {{"devServer": "bancho","scorePoint": {score}, "accuracy": {accuracy},"marvelous": {marvelous},"perfect": {perfect},"great": {great},"good": {good},"bad": {bad},"miss": {miss},"modList": "{mods}","maxCombo": {combo}}}, "pc": {{"macaddr": "temp","hwid": "temp"}} }}"#,
        token = get_token().unwrap(),
        md5 = get_beatmap_md5(p,state),
        score = crate::reader2::reader_resultscreen::get_result_score(p,state),
        accuracy = crate::reader2::reader_resultscreen::get_result_accuracy(p, state) * 100.0,
        marvelous = get_result_hit_geki(p,state),
        perfect = crate::reader2::reader_resultscreen::get_result_hit_300(p, state),
        great = get_result_hit_katu(p,state),
        good = get_result_hit_100(p,state),
        bad = get_result_hit_50(p,state),
        miss = get_result_hit_miss(p,state),
        mods =mods_to_string(get_mods(p,state)),
        combo = crate::reader2::reader_resultscreen::get_result_max_combo(p, state)
    );
    println!("Sending Score");
    println!("{:?}", body);
    // Appel de la fonction request avec les paramètres
    let url = "https://osef.me/api/sendScore"; // Remplace par ton URL
    let response = crate::utils::request2("POST", url, Some(body));

    // Gestion du résultat de la requête
    if (response.is_ok()) {
        println!("{:?}", "Succesfully added beatmap")
    } else {
        println!("{:?}", response.err())
    }
    true

}