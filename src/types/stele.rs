use std::io;

use serde::Serialize;

use crate::data::BinaryData;
use crate::decode::{Decode, DecodeState};

#[derive(Debug, Clone, Serialize)]
pub struct Stele {
    pub _0: i32,
    pub _1: i32,
    pub _2: bool,
    pub _3: String,
    pub _4: Vec<Stele_4>,
    pub _5: Vec<Stele_5>,
    pub _6: Vec<Stele_6>,
    pub _7: Vec<Stele_7>,
    pub _8: Vec<Stele_8>,
}

impl Decode for Stele {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let _0 = state.decode()?;
        let _1 = state.decode()?;
        let _2 = state.decode()?;
        let _3 = state.decode()?;
        let _4 = state.decode()?;
        let _5 = state.decode()?;
        let _6 = state.decode()?;
        let _7 = state.decode()?;
        let _8 = state.decode()?;
        Ok(Self {
            _0,
            _1,
            _2,
            _3,
            _4,
            _5,
            _6,
            _7,
            _8,
        })
    }
}

impl BinaryData for Stele {
    const TYPE_ID: i16 = 151;
}

#[derive(Debug, Clone, Serialize)]
pub struct Stele_4 {
    pub _0: i32,
    pub _1: i32,
    pub _2: i64,
}

impl Decode for Stele_4 {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let _0 = state.decode()?;
        let _1 = state.decode()?;
        let _2 = state.decode()?;
        Ok(Self { _0, _1, _2 })
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Stele_5 {
    pub _0: i32,
    pub _1: i32,
    pub _2: i64,
}

impl Decode for Stele_5 {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let _0 = state.decode()?;
        let _1 = state.decode()?;
        let _2 = state.decode()?;
        Ok(Self { _0, _1, _2 })
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Stele_6 {
    pub _0: i32,
    pub _1: i32,
    pub _2: i32,
}

impl Decode for Stele_6 {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let _0 = state.decode()?;
        let _1 = state.decode()?;
        let _2 = state.decode()?;
        Ok(Self { _0, _1, _2 })
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Stele_7 {
    pub _0: i32,
    pub _1: i32,
    pub _2: i32,
    pub _3: bool,
    pub _4: i32,
}

impl Decode for Stele_7 {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let _0 = state.decode()?;
        let _1 = state.decode()?;
        let _2 = state.decode()?;
        let _3 = state.decode()?;
        let _4 = state.decode()?;
        Ok(Self { _0, _1, _2, _3, _4 })
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Stele_8_4 {
    pub _0: i32,
    pub _1: i32,
    pub _2: i32,
    pub _3: bool,
    pub _4: i32,
}

impl Decode for Stele_8_4 {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let _0 = state.decode()?;
        let _1 = state.decode()?;
        let _2 = state.decode()?;
        let _3 = state.decode()?;
        let _4 = state.decode()?;
        Ok(Self { _0, _1, _2, _3, _4 })
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Stele_8_5 {
    pub _0: i32,
    pub _1: i32,
    pub _2: i32,
    pub _3: bool,
    pub _4: i32,
}

impl Decode for Stele_8_5 {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let _0 = state.decode()?;
        let _1 = state.decode()?;
        let _2 = state.decode()?;
        let _3 = state.decode()?;
        let _4 = state.decode()?;
        Ok(Self { _0, _1, _2, _3, _4 })
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Stele_8 {
    pub _0: i32,
    pub _1: i32,
    pub _2: i32,
    pub _3: i16,
    pub _4: Vec<Stele_8_4>,
    pub _5: Vec<Stele_8_5>,
}

impl Decode for Stele_8 {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let _0 = state.decode()?;
        let _1 = state.decode()?;
        let _2 = state.decode()?;
        let _3 = state.decode()?;
        let _4 = state.decode()?;
        let _5 = state.decode()?;
        Ok(Self {
            _0,
            _1,
            _2,
            _3,
            _4,
            _5,
        })
    }
}
