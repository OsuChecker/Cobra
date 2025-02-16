use std::path::{Path, PathBuf};
use image::ImageReader;
use reqwest::get;
use rosu_mem::process::{Process, ProcessTraits};
use slint::{Image, SharedPixelBuffer, SharedString};
use crate::MapData;
use crate::reader::structs::State;


pub(crate) fn read_from_beatmap_ptr_string(p: &Process, state: &mut State, offset: i32) -> eyre::Result<String>
{
    let beatmap_ptr = p.read_i32(state.addresses.base - 0xC)?;
    let beatmap_addr = p.read_i32(beatmap_ptr)?;
    Ok(p.read_string(beatmap_addr + offset)?)
}

pub(crate) fn read_multiple_from_beatmap_ptr_string(p: &Process, state: &mut State, offset: Vec<i32>) -> eyre::Result<Vec<String>>
{
    let beatmap_ptr = p.read_i32(state.addresses.base - 0xC)?;
    let beatmap_addr = p.read_i32(beatmap_ptr)?;
    let mut result = Vec::new();
    for offset in offset {
        result.push(p.read_string(beatmap_addr + offset)?);
    }
    Ok(result)
}

pub(crate) fn get_beatmap_md5(p: &Process, state: &mut State) -> eyre::Result<String>
{
    Ok(read_from_beatmap_ptr_string(p,state,0x6c)?)
}


pub(crate) fn get_author(p: &Process, state: &mut State) -> eyre::Result<String>
{
    Ok(read_from_beatmap_ptr_string(p,state,0x18)?)
}

pub(crate) fn get_creator(p: &Process, state: &mut State) -> eyre::Result<String>
{
    Ok(read_from_beatmap_ptr_string(p,state,0x7C)?)

}

pub(crate) fn get_title(p: &Process, state: &mut State) -> eyre::Result<String>
{
    Ok(read_from_beatmap_ptr_string(p,state,0x24)?)

}

pub(crate) fn get_difficulty(p: &Process, state: &mut State) -> eyre::Result<String>
{
    Ok(read_from_beatmap_ptr_string(p,state,0xAC)?)

}

pub(crate) fn get_beatmap(p: &Process, state: &mut State) -> eyre::Result<MapData>
{
    let mut vec = Vec::new();
    vec.push(0x18); // author
    vec.push(0x7C); // creator
    vec.push(0xAC); // difficulty
    vec.push(0x24); // title / song
    vec.push(0x6c); // md5

    let vec = read_multiple_from_beatmap_ptr_string(p,state,vec)?;
    Ok(MapData{
        author : SharedString::from(vec[0].clone()),
        cover : Image::default(),
        creator : SharedString::from(vec[1].clone()),
        difficulties: SharedString::from(vec[2].clone()),
        song : SharedString::from(vec[3].clone()),
        md5 : SharedString::from(vec[4].clone()),
        download_progress: 0.0,
        link : SharedString::from(String::new()),
    })

}



pub(crate) fn get_path_folder(p: &Process, state: &mut State) -> eyre::Result<String> {

    let settings_ptr = p.read_i32(state.addresses.settings+0x8)?;
    let settings_addr = p.read_i32(settings_ptr+0xb8)?;
    Ok(p.read_string(settings_addr+0x4)?)
}

pub(crate) fn get_folder(p: &Process, state: &mut State) -> eyre::Result<String> {
    let beatmap_ptr = p.read_i32(state.addresses.base - 0xC)?;
    let beatmap_addr = p.read_i32(beatmap_ptr)?;
    Ok(p.read_string(beatmap_addr + 0x78)?)
}
pub(crate) fn get_filename(p: &Process, state: &mut State) -> eyre::Result<String> {
    let beatmap_ptr = p.read_i32(state.addresses.base - 0xC)?;
    let beatmap_addr = p.read_i32(beatmap_ptr)?;
    Ok(p.read_string(beatmap_addr + 0x90)?)
}
pub(crate) fn get_beatmap_path(p: &Process, state: &mut State) -> eyre::Result<String>
{
    let path = get_path_folder(p,state)?;
    let song_path = Path::new(&path);
    let song_path = song_path.join(get_folder(p,state)?);
    let song_path = song_path.join(get_filename(p,state)?);
    Ok(song_path.display().to_string())
}

pub(crate) fn get_audio_path(p: &Process, state: &mut State) -> eyre::Result<String>
{
    let path = get_path_folder(p, state)?;
    let song_path = Path::new(&path);
    let song_path = song_path.join(get_folder(p, state)?);
    let song_path = song_path.join(read_from_beatmap_ptr_string(p,state,0x64)?);
    Ok(song_path.display().to_string())
}

pub(crate) fn get_beatmap_cover(p: &Process, state: &mut State) -> eyre::Result<String> {
    let beatmap_ptr = p.read_i32(state.addresses.base - 0xC)?;
    let beatmap_addr = p.read_i32(beatmap_ptr)?;
    Ok(p.read_string(beatmap_addr + 0x68)?)
}
pub(crate) fn get_cover_path(p: &Process, state: &mut State) -> eyre::Result<String> {
    let path = get_path_folder(p, state)?;
    let song_path = Path::new(&path);
    let song_path = song_path.join(get_folder(p, state)?);
    let song_path = song_path.join(get_beatmap_cover(p, state)?);

    Ok(song_path.display().to_string())
}

