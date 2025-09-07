use std::io;

use serde::Serialize;

use crate::data::BinaryData;
use crate::decode::{Decode, DecodeState};

#[derive(Debug, Clone, Serialize)]
pub struct AchievementLootList {
    pub _0: i32,
    pub _1: i16,
    pub _2: Vec<AchievementLootList_2>,
}

impl Decode for AchievementLootList {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let _0 = state.decode()?;
        let _1 = state.decode()?;
        let _2 = state.decode()?;
        Ok(Self { _0, _1, _2 })
    }
}

impl BinaryData for AchievementLootList {
    const TYPE_ID: i16 = 141;
}

#[derive(Debug, Clone, Serialize)]
pub struct AchievementLootList_2 {
    pub _0: i32,
    pub _1: f64,
    pub _2: String,
    pub _3: i32,
}

impl Decode for AchievementLootList_2 {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let _0 = state.decode()?;
        let _1 = state.decode()?;
        let _2 = state.decode()?;
        let _3 = state.decode()?;
        Ok(Self { _0, _1, _2, _3 })
    }
}
