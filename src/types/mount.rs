use std::io;

use serde::Serialize;

use crate::data::BinaryData;
use crate::decode::{Decode, DecodeState};

#[derive(Debug, Clone, Serialize)]
pub struct Mount {
    pub id: i32,
    pub item_ref_id: i32,
    pub gfx_id: i32,
    pub _3: i32,
    pub _4: i16,
    pub _5: i16,
    pub _6: i8,
    pub _7: i64,
    pub _8: i64,
    pub _9: i32,
    pub _10: Vec<Mount_10>,
    pub _11: Vec<Mount_11>,
    pub _12: Vec<Mount_12>,
}

impl Decode for Mount {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        let item_ref_id = state.decode()?;
        let gfx_id = state.decode()?;
        let _3 = state.decode()?;
        let _4 = state.decode()?;
        let _5 = state.decode()?;
        let _6 = state.decode()?;
        let _7 = state.decode()?;
        let _8 = state.decode()?;
        let _9 = state.decode()?;
        let _10 = state.decode()?;
        let _11 = state.decode()?;
        let _12 = state.decode()?;
        Ok(Self {
            id,
            item_ref_id,
            gfx_id,
            _3,
            _4,
            _5,
            _6,
            _7,
            _8,
            _9,
            _10,
            _11,
            _12,
        })
    }
}

impl BinaryData for Mount {
    const TYPE_ID: i16 = 144;
}

#[derive(Debug, Clone, Serialize)]
pub struct Mount_10 {
    pub _0: i8,
    pub _1: i8,
}

impl Decode for Mount_10 {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let _0 = state.decode()?;
        let _1 = state.decode()?;
        Ok(Self { _0, _1 })
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Mount_11 {
    pub _0: i32,
    pub _1: i32,
}

impl Decode for Mount_11 {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let _0 = state.decode()?;
        let _1 = state.decode()?;
        Ok(Self { _0, _1 })
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Mount_12 {
    pub _0: i32,
    pub _1: bool,
}

impl Decode for Mount_12 {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let _0 = state.decode()?;
        let _1 = state.decode()?;
        Ok(Self { _0, _1 })
    }
}
