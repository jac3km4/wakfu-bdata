use std::io;

use serde::Serialize;

use crate::data::BinaryData;
use crate::decode::{Decode, DecodeState};

#[derive(Debug, Clone, Serialize)]
pub struct WorldLootList {
    pub _0: i32,
    pub _1: f64,
    pub _2: i16,
    pub _3: i16,
    pub _4: String,
    pub _5: i16,
    pub _6: Vec<WorldLootList_6>,
}

impl Decode for WorldLootList {
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

impl BinaryData for WorldLootList {
    const TYPE_ID: i16 = 75;
}

#[derive(Debug, Clone, Serialize)]
pub struct WorldLootList_6 {
    pub _0: i32,
    pub _1: f64,
    pub _2: String,
    pub _3: i16,
    pub _4: i16,
    pub _5: i16,
    pub _6: i16,
    pub _7: i16,
    pub _8: bool,
}

impl Decode for WorldLootList_6 {
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
