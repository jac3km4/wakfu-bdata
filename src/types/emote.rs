use std::io;

use serde::Serialize;

use crate::data::BinaryData;
use crate::decode::{Decode, DecodeState};

#[derive(Debug, Clone, Serialize)]
pub struct Emote {
    pub id: i32,
    pub kind: i16,
    pub cmd: String,
    pub need_target: bool,
    pub move_to_target: bool,
    pub infinite_duration: bool,
    pub script_params: Vec<String>,
    pub _7: i32,
    pub _8: i32,
    pub _9: String,
    pub _10: Vec<i32>,
    pub _11: Vec<i32>,
}

impl Decode for Emote {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        let kind = state.decode()?;
        let cmd = state.decode()?;
        let need_target = state.decode()?;
        let move_to_target = state.decode()?;
        let infinite_duration = state.decode()?;
        let script_params = state.decode()?;
        let _7 = state.decode()?;
        let _8 = state.decode()?;
        let _9 = state.decode()?;
        let _10 = state.decode()?;
        let _11 = state.decode()?;
        Ok(Self {
            id,
            kind,
            cmd,
            need_target,
            move_to_target,
            infinite_duration,
            script_params,
            _7,
            _8,
            _9,
            _10,
            _11,
        })
    }
}

impl BinaryData for Emote {
    const TYPE_ID: i16 = 81;
}
