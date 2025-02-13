use rosu_mem::process::{Process, ProcessTraits};
use crate::reader::structs::State;

/// Reads the current score during gameplay from memory
///
/// # Arguments
/// * `p` - Process handle for osu!
/// * `state` - Current state containing memory addresses
///
/// # Returns
/// Current score as i32
pub fn get_score_gameplay(p: &Process, state: &mut State) -> i32 {
    let ruleset_addr = p.read_i32(state.addresses.rulesets - 0xb).unwrap();
    let ruleset_addr = p.read_i32(ruleset_addr + 0x4).unwrap();
    let gameplay_base = p.read_i32(ruleset_addr + 0x68).unwrap();
    let score_base = p.read_i32(gameplay_base + 0x38).unwrap();
    p.read_i32(score_base + 0x78).unwrap()
}

/// Reads currently active mods from memory
///
/// # Arguments
/// * `p` - Process handle for osu!
/// * `state` - Current state containing memory addresses
///
/// # Returns
/// Active mods as u32 bitmask
pub fn get_mods(p: &Process, state: &mut State) -> u32 {
    let ruleset_addr = p.read_i32(state.addresses.rulesets - 0xb).unwrap();
    let ruleset_addr = p.read_i32(ruleset_addr + 0x4).unwrap();
    let resultscreen_base = p.read_i32(ruleset_addr + 0x38).unwrap();
    let mods_xor_base = p.read_i32(resultscreen_base + 0x1C).unwrap();
    let mods_xor1 = p.read_u64(mods_xor_base + 0xc).unwrap();
    let mods_xor2 =  p.read_u64(mods_xor_base + 0x8).unwrap();
    (mods_xor1 ^ mods_xor2) as u32
}

/// Reads current combo from memory
///
/// # Arguments
/// * `p` - Process handle for osu!
/// * `state` - Current state containing memory addresses
///
/// # Returns
/// Current combo as i16
pub fn get_combo(p: &Process, state: &mut State) -> i16 {
    let ruleset_addr = p.read_i32(state.addresses.rulesets - 0xb).unwrap();
    let ruleset_addr = p.read_i32(ruleset_addr + 0x4).unwrap();
    let gameplay_base = p.read_i32(ruleset_addr + 0x68).unwrap();
    let score_base = p.read_i32(gameplay_base + 0x38).unwrap();
    p.read_i16(score_base + 0x94).unwrap()
}

/// Reads maximum combo achieved in current play from memory
///
/// # Arguments
/// * `p` - Process handle for osu!
/// * `state` - Current state containing memory addresses
///
/// # Returns
/// Maximum combo as i16
pub fn get_max_combo(p: &Process, state: &mut State) -> i16 {
    let ruleset_addr = p.read_i32(state.addresses.rulesets - 0xb).unwrap();
    let ruleset_addr = p.read_i32(ruleset_addr + 0x4).unwrap();
    let gameplay_base = p.read_i32(ruleset_addr + 0x68).unwrap();
    let score_base = p.read_i32(gameplay_base + 0x38).unwrap();
    p.read_i16(score_base + 0x68).unwrap()
}


/// Reads current HP value from memory
///
/// # Arguments
/// * `p` - Process handle for osu!
/// * `state` - Current state containing memory addresses
///
/// # Returns
/// Current HP as f64 (0.0 to 1.0)
pub fn get_current_hp(p: &Process, state: &mut State) -> f64 {
    let ruleset_addr = p.read_i32(state.addresses.rulesets - 0xb).unwrap();
    let ruleset_addr = p.read_i32(ruleset_addr + 0x4).unwrap();
    let gameplay_base = p.read_i32(ruleset_addr + 0x68).unwrap();
    let hp_base = p.read_i32(gameplay_base + 0x40).unwrap();
    p.read_f64(hp_base + 0x1C).unwrap()
}

/// Reads current player username from memory
///
/// # Arguments
/// * `p` - Process handle for osu!
/// * `state` - Current state containing memory addresses
///
/// # Returns
/// Username as String
pub fn get_username(p: &Process, state: &mut State) -> String {
    let ruleset_addr = p.read_i32(state.addresses.rulesets - 0xb).unwrap();
    let ruleset_addr = p.read_i32(ruleset_addr + 0x4).unwrap();
    let gameplay_base = p.read_i32(ruleset_addr + 0x68).unwrap();
    let score_base = p.read_i32(gameplay_base + 0x38).unwrap();
    p.read_string(score_base + 0x28).unwrap()
}


/// Reads in-game time (play time) from memory
///
/// # Arguments
/// * `p` - Process handle for osu!
/// * `state` - Current state containing memory addresses
///
/// # Returns
/// In-game time in milliseconds as i32
pub fn get_ig_time(p: &Process, state: &mut State) -> i32 {
    let playtime_ptr = p.read_i32(state.addresses.playtime + 0x5).unwrap();
    p.read_i32(playtime_ptr).unwrap()
}

/// Reads number of retries in current session from memory
///
/// # Arguments
/// * `p` - Process handle for osu!
/// * `state` - Current state containing memory addresses
///
/// # Returns
/// Number of retries as i32
pub fn get_retries(p: &Process, state: &mut State) -> i32 {
    let igt_addr = p.read_i32(state.addresses.base - 0x33).unwrap();
    p.read_i32(igt_addr+0x8).unwrap()
}