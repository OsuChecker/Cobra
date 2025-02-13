use std::collections::HashMap;
use rosu_mem::process::{Process, ProcessTraits};
use crate::reader::structs::{Hit, State};

pub fn get_result_username(p: &Process, state: &mut State) -> String {
    let ruleset_addr = p.read_i32(state.addresses.rulesets - 0xb).unwrap();
    let ruleset_addr = p.read_i32(ruleset_addr + 0x4).unwrap();
    let result_base = p.read_i32(ruleset_addr + 0x38).unwrap();
    return p.read_string(result_base + 0x28).unwrap();
}
pub fn get_result_score(p: &Process, state: &mut State) -> i32 {
    let ruleset_addr = p.read_i32(state.addresses.rulesets - 0xb).unwrap();
    let ruleset_addr = p.read_i32(ruleset_addr + 0x4).unwrap();
    let result_base = p.read_i32(ruleset_addr + 0x38).unwrap();
    return p.read_i32(result_base + 0x78).unwrap();
}
pub fn get_result_mode(p: &Process, state: &mut State) -> u8 {
    let ruleset_addr = p.read_i32(state.addresses.rulesets - 0xb).unwrap();
    let ruleset_addr = p.read_i32(ruleset_addr + 0x4).unwrap();
    let result_base = p.read_i32(ruleset_addr + 0x38).unwrap();
    return p.read_i32(result_base + 0x64).unwrap() as u8;
}

pub fn get_result_hit_300(p: &Process, state: &mut State) -> i16 {
    let ruleset_addr = p.read_i32(state.addresses.rulesets - 0xb).unwrap();
    let ruleset_addr = p.read_i32(ruleset_addr + 0x4).unwrap();
    let result_base = p.read_i32(ruleset_addr + 0x38).unwrap();
    return p.read_i16(result_base + 0x8A).unwrap();
}
pub fn get_result_hit_100(p: &Process, state: &mut State) -> i16 {
    let ruleset_addr = p.read_i32(state.addresses.rulesets - 0xb).unwrap();
    let ruleset_addr = p.read_i32(ruleset_addr + 0x4).unwrap();
    let result_base = p.read_i32(ruleset_addr + 0x38).unwrap();
    return p.read_i16(result_base + 0x88).unwrap();
}
pub fn get_result_hit_50(p: &Process, state: &mut State) -> i16 {
    let ruleset_addr = p.read_i32(state.addresses.rulesets - 0xb).unwrap();
    let ruleset_addr = p.read_i32(ruleset_addr + 0x4).unwrap();
    let result_base = p.read_i32(ruleset_addr + 0x38).unwrap();
    return p.read_i16(result_base + 0x8C).unwrap();
}
pub fn get_result_hit_geki(p: &Process, state: &mut State) -> i16 {
    let ruleset_addr = p.read_i32(state.addresses.rulesets - 0xb).unwrap();
    let ruleset_addr = p.read_i32(ruleset_addr + 0x4).unwrap();
    let result_base = p.read_i32(ruleset_addr + 0x38).unwrap();
    return p.read_i16(result_base + 0x8E).unwrap();
}
pub fn get_result_hit_katu(p: &Process, state: &mut State) -> i16 {
    let ruleset_addr = p.read_i32(state.addresses.rulesets - 0xb).unwrap();
    let ruleset_addr = p.read_i32(ruleset_addr + 0x4).unwrap();
    let result_base = p.read_i32(ruleset_addr + 0x38).unwrap();
    return p.read_i16(result_base + 0x90).unwrap();
}
pub fn get_result_hit_miss(p: &Process, state: &mut State) -> i16 {
    let ruleset_addr = p.read_i32(state.addresses.rulesets - 0xb).unwrap();
    let ruleset_addr = p.read_i32(ruleset_addr + 0x4).unwrap();
    let result_base = p.read_i32(ruleset_addr + 0x38).unwrap();
    return p.read_i16(result_base + 0x92).unwrap();
}


pub fn get_result_hits(p: &Process, state: &mut State) -> Hit {
    let ruleset_addr = p.read_i32(state.addresses.rulesets - 0xb).unwrap();
    let ruleset_addr = p.read_i32(ruleset_addr + 0x4).unwrap();
    let base = p.read_i32(ruleset_addr + 0x38).unwrap();
    Hit {
        _300: p.read_i16(base + 0x8A).unwrap(),
        _100: p.read_i16(base + 0x88).unwrap(),
        _50: p.read_i16(base + 0x8C).unwrap(),
        _miss: p.read_i16(base + 0x92).unwrap(),
        _geki: p.read_i16(base + 0x8E).unwrap(),
        _katu: p.read_i16(base + 0x90).unwrap(),
    }
}
fn calculate_accuracy(
    gamemode: u8,
    hit: Hit
) -> f64 {
    let (numerator, denominator) = match gamemode {
        0 => (
                hit._300 as f64 * 6.0 + hit._100 as f64 * 2.0 + hit._50 as f64,
                (hit._300 + hit._100 + hit._50 + hit._miss) as f64 * 6.0
        ),
        1 => (
                hit._300 as f64 * 2.0 + hit._100 as f64,
                (hit._300 + hit._100 + hit._50 + hit._miss) as f64 * 2.0
        ),
        2 => (
                (hit._300 + hit._100 + hit._50) as f64,
                (hit._300 + hit._100 + hit._50 + hit._katu + hit._miss) as f64
        ),
        3 => (
                (hit._geki + hit._300) as f64 * 6.0 + hit._katu as f64 * 4.0 + hit._100 as f64 * 2.0 + hit._50 as f64,
                (hit._geki + hit._300 + hit._katu + hit._100 + hit._50 + hit._miss) as f64 * 6.0
        ),
        _ => panic!("Unsupported Gamemode : {}", gamemode)
    };

    numerator / denominator
}

pub fn get_result_accuracy(p: &Process, state: &mut State) -> f64 {
    calculate_accuracy(
        get_result_mode(p, state),
        get_result_hits(p,state),
    )
}
pub fn get_result_max_combo(p: &Process, state: &mut State) -> i16 {
    let ruleset_addr = p.read_i32(state.addresses.rulesets - 0xb).unwrap();
    let ruleset_addr = p.read_i32(ruleset_addr + 0x4).unwrap();
    let score_base = p.read_i32(ruleset_addr + 0x38).unwrap();
    p.read_i16(score_base + 0x68).unwrap()
}