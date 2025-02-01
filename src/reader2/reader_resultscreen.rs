use crate::structs::{ OutputValues, State};
use rosu_mem::process::{Process, ProcessTraits};

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

fn calculate_accuracy(
    gamemode: u8,
    hit_300: i16,
    hit_100: i16,
    hit_50: i16,
    hit_geki: i16,
    hit_katu: i16,
    hit_miss: i16,
) -> f64 {
    match gamemode {
        0 => {
            (hit_300 as f64 * 6. + hit_100 as f64 * 2. + hit_50 as f64)
                / ((hit_300 + hit_100 + hit_50 + hit_miss) as f64 * 6.)
        }
        1 => {
            (hit_300 as f64 * 2. + hit_100 as f64)
                / ((hit_300 + hit_100 + hit_50 + hit_miss) as f64 * 2.)
        }
        2 => {
            (hit_300 + hit_100 + hit_50) as f64
                / (hit_300 + hit_100 + hit_50 + hit_katu + hit_miss) as f64
        }
        3 => {
            ((hit_geki + hit_300) as f64 * 6.
                + hit_katu as f64 * 4.
                + hit_100 as f64 * 2.
                + hit_50 as f64)
                / ((hit_geki + hit_300 + hit_katu + hit_100 + hit_50 + hit_miss) as f64 * 6.)
        }
        _ => {
            panic!("Unsupported gamemode: {}", gamemode); // Gestion des cas imprévus
        }
    }
}
pub fn get_result_accuracy(p: &Process, state: &mut State) -> f64 {
    calculate_accuracy(
        get_result_mode(p, state),
        get_result_hit_300(p, state),
        get_result_hit_100(p, state),
        get_result_hit_50(p, state),
        get_result_hit_geki(p, state),
        get_result_hit_katu(p, state),
        get_result_hit_miss(p, state),
    )
}
pub fn get_result_max_combo(p: &Process, state: &mut State) -> i16 {
    let ruleset_addr = p.read_i32(state.addresses.rulesets - 0xb).unwrap();
    let ruleset_addr = p.read_i32(ruleset_addr + 0x4).unwrap();
    let score_base = p.read_i32(ruleset_addr + 0x38).unwrap();
    return p.read_i16(score_base + 0x68).unwrap();
}