use crate::BinaryData;
use crate::decode::{Decode, DecodeState};
use std::io;
use std::marker::PhantomData;

#[derive(Debug, Clone, serde::Serialize)]
pub struct LightLootList {
    pub id: i32,
    pub _1: Vec<LightLootList_1>,
}

impl BinaryData for LightLootList {
    fn id(_phantom: PhantomData<Self>) -> i32 {
        114
    }
}

impl Decode for LightLootList {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        let _1 = state.decode()?;
        Ok(LightLootList { id, _1 })
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct LightLootList_1 {
    pub _0: i32,
    pub _1: i16,
    pub _2: i16,
    pub _3: i16,
    pub _4: bool,
}

impl Decode for LightLootList_1 {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let _0 = state.decode()?;
        let _1 = state.decode()?;
        let _2 = state.decode()?;
        let _3 = state.decode()?;
        let _4 = state.decode()?;
        Ok(LightLootList_1 { _0, _1, _2, _3, _4 })
    }
}
