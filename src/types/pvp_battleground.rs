use crate::BinaryData;
use crate::decode::{Decode, DecodeState};
use std::io;
use std::marker::PhantomData;

#[derive(Debug, Clone, serde::Serialize)]
pub struct PvpBattleground {
    pub _0: i32,
    pub _1: i32,
    pub _2: Vec<PvpBattleground_2>,
    pub _3: Vec<PvpBattleground_3>,
}

impl BinaryData for PvpBattleground {
    fn id(_phantom: PhantomData<Self>) -> i32 {
        140
    }
}

impl Decode for PvpBattleground {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let _0 = state.decode()?;
        let _1 = state.decode()?;
        let _2 = state.decode()?;
        let _3 = state.decode()?;
        Ok(PvpBattleground { _0, _1, _2, _3 })
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct PvpBattleground_3 {
    pub _0: i32,
    pub _1: i32,
}

impl Decode for PvpBattleground_3 {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let _0 = state.decode()?;
        let _1 = state.decode()?;
        Ok(PvpBattleground_3 { _0, _1 })
    }
}

#[derive(Debug, Clone, serde::Serialize)]
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
        Ok(PvpBattleground_2 {
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
