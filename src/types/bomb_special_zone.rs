use crate::BinaryData;
use crate::decode::{Decode, DecodeState};
use std::io;
use std::marker::PhantomData;

#[derive(Debug, Clone, serde::Serialize)]
pub struct BombSpecialZone {
    pub _0: i32,
    pub _1: i32,
    pub _2: Vec<BombSpecialZone_2>,
    pub _3: Vec<BombSpecialZone_3>,
    pub _4: Vec<i32>,
}

impl BinaryData for BombSpecialZone {
    fn id(_phantom: PhantomData<Self>) -> i32 {
        10
    }
}

impl Decode for BombSpecialZone {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let _0 = state.decode()?;
        let _1 = state.decode()?;
        let _2 = state.decode()?;
        let _3 = state.decode()?;
        let _4 = state.decode()?;
        Ok(BombSpecialZone { _0, _1, _2, _3, _4 })
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct BombSpecialZone_3 {}

impl Decode for BombSpecialZone_3 {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        Ok(BombSpecialZone_3 {})
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct BombSpecialZone_2 {}

impl Decode for BombSpecialZone_2 {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        Ok(BombSpecialZone_2 {})
    }
}
