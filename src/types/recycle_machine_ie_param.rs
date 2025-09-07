use std::io;

use serde::Serialize;

use crate::data::BinaryData;
use crate::decode::{Decode, DecodeState};

#[derive(Debug, Clone, Serialize)]
pub struct RecycleMachineIeParam {
    pub id: i32,
    pub visual_mru_id: i32,
    pub _2: RecycleMachineIeParam_2,
}

impl Decode for RecycleMachineIeParam {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        let visual_mru_id = state.decode()?;
        let _2 = state.decode()?;
        Ok(Self {
            id,
            visual_mru_id,
            _2,
        })
    }
}

impl BinaryData for RecycleMachineIeParam {
    const TYPE_ID: i16 = 85;
}

#[derive(Debug, Clone, Serialize)]
pub struct RecycleMachineIeParam_2 {
    pub _0: i8,
    pub _1: i32,
}

impl Decode for RecycleMachineIeParam_2 {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let _0 = state.decode()?;
        let _1 = state.decode()?;
        Ok(Self { _0, _1 })
    }
}
