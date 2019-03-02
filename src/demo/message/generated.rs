/// Messages that consists only of primitives and string and can be derived
use bitstream_reader::BitRead;
use std::collections::HashMap;

#[derive(BitRead, Debug)]
pub struct FileMessage {
    pub transfer_id: u32,
    pub file_name: String,
    pub requested: bool,
}

#[derive(BitRead, Debug)]
pub struct NetTickMessage {
    pub tick: u32,
    pub frame_time: u16,
    pub std_dev: u16,
}

#[derive(BitRead, Debug)]
pub struct StringCmdMessage {
    pub command: String,
}

#[derive(BitRead, Debug)]
pub struct SigOnStateMessage {
    pub state: u8,
    pub count: u32,
}

#[derive(BitRead, Debug)]
pub struct PrintMessage {
    pub value: String,
}

#[derive(BitRead, Debug)]
pub struct ServerInfoMessage {
    pub version: u16,
    pub server_count: u32,
    pub stv: bool,
    pub dedicated: bool,
    pub max_crc: u32,
    pub max_classes: u16,
    pub map_hash: u128,
    pub player_count: u8,
    pub max_player_count: u8,
    pub interval_per_tick: f32,
    #[size = 1]
    pub platform: String,
    pub game: String,
    pub map: String,
    pub skybox: String,
    pub server_name: String,
    pub replay: bool,
}

#[derive(BitRead, Debug)]
pub struct SetPauseMessage {
    pub pause: bool,
}

#[derive(BitRead, Debug)]
pub struct SetViewMessage {
    #[size = 11]
    pub index: u16,
}

#[derive(BitRead, Debug)]
pub struct FixAngleMessage {
    pub relative: bool,
    pub x: u16,
    pub y: u16,
    pub z: u16,
}

#[derive(BitRead, Debug)]
pub struct PreFetchMessage {
    #[size = 14]
    pub index: u16,
}

#[derive(BitRead, Debug)]
pub struct GetCvarMessage {
    pub cookie: u32,
    pub value: String,
}

#[derive(BitRead, Debug)]
pub struct SetConVarMessage {
    #[size_bits = 8]
    vars: HashMap<String, String>,
}