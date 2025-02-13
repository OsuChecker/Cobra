use std::str::FromStr;
use rosu_mem::{
    process::{Process, ProcessTraits},
    signature::Signature,
};


#[derive(Default)]
pub struct StaticAddresses {
    pub base: i32,
    pub status: i32,
    pub menu_mods: i32,
    pub rulesets: i32,
    pub playtime: i32,
    pub skin: i32,
    pub chat_checker: i32,
    pub audio_time_base: i32,
    pub ig_time_base : i32,
}

pub struct Hit{
    pub _geki:i16,
    pub _300:i16,
    pub _katu:i16,
    pub _100:i16,
    pub _50:i16,
    pub _miss:i16,
}

impl StaticAddresses {
    pub fn new(p: &Process) -> Result<Self,Box<dyn std::error::Error>> {

        let base_sign = Signature::from_str("F8 01 74 04 83 65")?;
        let status_sign = Signature::from_str("48 83 F8 04 73 1E")?;
        let menu_mods_sign =
            Signature::from_str("C8 FF ?? ?? ?? ?? ?? 81 0D ?? ?? ?? ?? 00 08 00 00")?;

        let rulesets_sign = Signature::from_str("7D 15 A1 ?? ?? ?? ?? 85 C0")?;

        let playtime_sign = Signature::from_str("5E 5F 5D C3 A1 ?? ?? ?? ?? 89 ?? 04")?;

        let skin_sign = Signature::from_str("75 21 8B 1D")?;

        let chat_checker = Signature::from_str("0A D7 23 3C 00 00 ?? 01")?;

        let audio_time_base = Signature::from_str("DB 5C 24 34 8B 44 24 34")?;

        let ig_time_base = Signature::from_str("EB 0A A1 ?? ?? ?? ?? A3")?;
        Ok(Self {
            base: p.read_signature(&base_sign)?,
            status: p.read_signature(&status_sign)?,
            menu_mods: p.read_signature(&menu_mods_sign)?,
            rulesets: p.read_signature(&rulesets_sign)?,
            playtime: p.read_signature(&playtime_sign)?,
            skin: p.read_signature(&skin_sign)?,
            chat_checker: p.read_signature(&chat_checker)?,
            audio_time_base: p.read_signature(&audio_time_base)?,
            ig_time_base: p.read_signature(&ig_time_base)?,
        })
    }
}

/* for pp counter for later
#[derive(Default)]
pub struct InnerValues {
    pub gradual_performance_current: Option<GradualPerformance<'static>>,
    pub current_beatmap_perf: Option<PerformanceAttributes>,
}


impl InnerValues {
    pub fn reset(&mut self) {
        self.current_beatmap_perf = None;
        self.gradual_performance_current = None;
    }
}
*/

pub struct State {
    pub addresses: StaticAddresses,
}
