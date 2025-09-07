use std::io;

use serde::Serialize;

use crate::data::BinaryData;
use crate::decode::{Decode, DecodeState};

#[derive(Debug, Clone, Serialize)]
pub struct MonsterBlahblah {
    pub _0: i32,
    pub _1: i32,
    pub _2: i32,
    pub _3: i8,
    pub _4: Vec<MonsterBlahblah_4>,
}

impl Decode for MonsterBlahblah {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let _0 = state.decode()?;
        let _1 = state.decode()?;
        let _2 = state.decode()?;
        let _3 = state.decode()?;
        let _4 = state.decode()?;
        Ok(Self { _0, _1, _2, _3, _4 })
    }
}

impl BinaryData for MonsterBlahblah {
    const TYPE_ID: i16 = 43;
}

#[derive(Debug, Clone, Serialize)]
pub struct MonsterBlahblah_4 {
    pub _0: i32,
    pub _1: i8,
    pub _2: bool,
    pub _3: String,
}

impl Decode for MonsterBlahblah_4 {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let _0 = state.decode()?;
        let _1 = state.decode()?;
        let _2 = state.decode()?;
        let _3 = state.decode()?;
        Ok(Self { _0, _1, _2, _3 })
    }
}
