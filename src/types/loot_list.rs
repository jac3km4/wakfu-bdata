use crate::BinaryData;
use crate::decode::{Decode, DecodeState};
use std::io;
use std::marker::PhantomData;

#[derive(Debug, Clone, serde::Serialize)]
pub struct LootList {
    pub _0: i32,
    pub _1: i16,
    pub _2: Vec<LootList_2>,
}

impl BinaryData for LootList {
    fn id(_phantom: PhantomData<Self>) -> i32 {
        39
    }
}

impl Decode for LootList {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let _0 = state.decode()?;
        let _1 = state.decode()?;
        let _2 = state.decode()?;
        Ok(LootList { _0, _1, _2 })
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct LootList_2 {
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

impl Decode for LootList_2 {
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
        Ok(LootList_2 {
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
