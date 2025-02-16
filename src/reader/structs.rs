use std::str::FromStr;
use rosu_mem::{
    process::{Process, ProcessTraits},
    signature::Signature,
};


#[derive(Default, Clone)]
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
    pub settings : i32,
}

#[derive(Debug, Default)]
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

        let settings_base = Signature::from_str("83 E0 20 85 C0 7E 2F")?;
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
            settings: p.read_signature(&settings_base)?,
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
#[derive(Default,Clone)]
pub struct State {
    pub addresses: StaticAddresses,
}

#[derive( Debug, Default, PartialEq, Eq, Clone, Copy)]
#[repr(u32)]
pub enum GameState {
    PreSongSelect = 0,
    Playing = 2,
    SongSelect = 5,
    EditorSongSelect = 4,
    ResultScreen = 7,
    MultiplayerLobbySelect = 11,
    MultiplayerLobby = 12,
    MultiplayerResultScreen = 14,

    #[default]
    Unknown,
}

impl From<u32> for GameState {
    fn from(value: u32) -> Self {
        match value {
            0 => Self::PreSongSelect,
            2 => Self::Playing,
            4 => Self::EditorSongSelect,
            5 => Self::SongSelect,
            7 => Self::ResultScreen,
            11 => Self::MultiplayerLobbySelect,
            12 => Self::MultiplayerLobby,
            14 => Self::MultiplayerResultScreen,
            _ => Self::Unknown,
        }
    }
}


#[derive(Debug, Default, PartialEq, Eq, Copy, Clone)]
#[repr(i16)]
pub enum BeatmapStatus {
    #[default]
    Unknown = 0,
    Unsubmitted = 1,
    Unranked = 2,
    Unused = 3,
    Ranked = 4,
    Approved = 5,
    Qualified = 6,
    Loved = 7,
}

impl From<i16> for BeatmapStatus {
    fn from(value: i16) -> Self {
        match value {
            1 => Self::Unsubmitted,
            2 => Self::Unranked,
            3 => Self::Unused,
            4 => Self::Ranked,
            5 => Self::Approved,
            6 => Self::Qualified,
            7 => Self::Loved,
            _ => Self::Unknown,
        }
    }
}


#[derive(Debug, Default)]
pub struct ResultScreenValues {
    pub username: String,
    pub mode: u8,
    pub max_combo: i16,
    pub score: i32,
    pub hit : Hit,
    pub accuracy: f64,
}
