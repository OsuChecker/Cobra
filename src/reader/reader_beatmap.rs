use reqwest::get;
use rosu_mem::process::{Process, ProcessTraits};
use slint::{Image, SharedString};
use crate::MapData;
use crate::reader::structs::State;

pub(crate) fn get_beatmap_md5(p: &Process, state: &mut State) -> String {
    let ruleset_addr = p.read_i32(state.addresses.base - 0xC).unwrap();
    let ruleset_addr = p.read_i32(ruleset_addr ).unwrap();
    p.read_string(ruleset_addr + 0x6c).unwrap()
}


pub(crate) fn get_author(p: &Process, state: &mut State) -> String {
    let beatmap_ptr = p.read_i32(state.addresses.base - 0xC).unwrap();
    let beatmap_addr = p.read_i32(beatmap_ptr).unwrap();
    p.read_string(beatmap_addr + 0x18).unwrap()
}

pub(crate) fn get_creator(p: &Process, state: &mut State) -> String {
    let beatmap_ptr = p.read_i32(state.addresses.base - 0xC).unwrap();
    let beatmap_addr = p.read_i32(beatmap_ptr).unwrap();
    p.read_string(beatmap_addr + 0x7C).unwrap()
}

pub(crate) fn get_title(p: &Process, state: &mut State) -> String {
    let beatmap_ptr = p.read_i32(state.addresses.base - 0xC).unwrap();
    let beatmap_addr = p.read_i32(beatmap_ptr).unwrap();
    p.read_string(beatmap_addr + 0x24).unwrap()
}
pub(crate) fn get_difficulty(p: &Process, state: &mut State) -> String {
    let beatmap_ptr = p.read_i32(state.addresses.base - 0xC).unwrap();
    let beatmap_addr = p.read_i32(beatmap_ptr).unwrap();
    p.read_string(beatmap_addr + 0xAC).unwrap()
}

pub(crate) fn get_beatmap(p: &Process, state: &mut State) -> crate::MapData {
    MapData{
        author : SharedString::from(get_author(p, state)),
        cover : Image::default(),
        creator : SharedString::from(get_creator(p, state)),
        difficulties: SharedString::from(get_difficulty(p, state)),
        song : SharedString::from(get_title(p, state)),
        md5 : SharedString::from(get_beatmap_md5(p,state)),
        download_progress: 0.0,
        link : SharedString::from(String::new()),
    }

}