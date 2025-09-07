use std::io;

use serde::Serialize;

use crate::data::BinaryData;
use crate::decode::{Decode, DecodeState};

#[derive(Debug, Clone, Serialize)]
pub struct InfiniteWaves {
    pub _0: i32,
    pub _1: i32,
    pub _2: i32,
    pub _3: i32,
    pub _4: Vec<InfiniteWaves_4>,
}

impl Decode for InfiniteWaves {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let _0 = state.decode()?;
        let _1 = state.decode()?;
        let _2 = state.decode()?;
        let _3 = state.decode()?;
        let _4 = state.decode()?;
        Ok(Self { _0, _1, _2, _3, _4 })
    }
}

impl BinaryData for InfiniteWaves {
    const TYPE_ID: i16 = 147;
}

#[derive(Debug, Clone, Serialize)]
pub struct InfiniteWaves_4 {
    pub _0: i64,
    pub _1: i16,
}

impl Decode for InfiniteWaves_4 {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let _0 = state.decode()?;
        let _1 = state.decode()?;
        Ok(Self { _0, _1 })
    }
}
