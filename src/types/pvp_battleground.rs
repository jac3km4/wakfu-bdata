use std::io;

use serde::Serialize;

use crate::data::BinaryData;
use crate::decode::{Decode, DecodeState};

#[derive(Debug, Clone, Serialize)]
pub struct PvpBattleground {
    pub _0: i32,
    pub _1: i32,
    pub _2: Vec<PvpBattleground_2>,
    pub _3: Vec<PvpBattleground_3>,
}

impl Decode for PvpBattleground {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let _0 = state.decode()?;
        let _1 = state.decode()?;
        let _2 = state.decode()?;
        let _3 = state.decode()?;
        Ok(Self { _0, _1, _2, _3 })
    }
}

impl BinaryData for PvpBattleground {
    const TYPE_ID: i16 = 140;
}

#[derive(Debug, Clone, Serialize)]
pub struct PvpBattleground_2 {
    pub _0: i32,
    pub _1: i32,
    pub _2: i32,
    pub _3: i32,
    pub _4: i32,
    pub _5: i32,
    pub _6: i32,
}

impl Decode for PvpBattleground_2 {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let _0 = state.decode()?;
        let _1 = state.decode()?;
        let _2 = state.decode()?;
        let _3 = state.decode()?;
        let _4 = state.decode()?;
        let _5 = state.decode()?;
        let _6 = state.decode()?;
        Ok(Self {
            _0,
            _1,
            _2,
            _3,
            _4,
            _5,
            _6,
        })
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct PvpBattleground_3 {
    pub _0: i32,
    pub _1: i32,
}

impl Decode for PvpBattleground_3 {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let _0 = state.decode()?;
        let _1 = state.decode()?;
        Ok(Self { _0, _1 })
    }
}
