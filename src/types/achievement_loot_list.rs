use crate::BinaryData;
use crate::decode::{Decode, DecodeState};
use std::io;
use std::marker::PhantomData;

#[derive(Debug, Clone, serde::Serialize)]
pub struct AchievementLootList {
    pub _0: i32,
    pub _1: i16,
    pub _2: Vec<AchievementLootList_2>,
}

impl BinaryData for AchievementLootList {
    fn id(_phantom: PhantomData<Self>) -> i32 {
        141
    }
}

impl Decode for AchievementLootList {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let _0 = state.decode()?;
        let _1 = state.decode()?;
        let _2 = state.decode()?;
        Ok(AchievementLootList { _0, _1, _2 })
    }
}

#[derive(Debug, Clone, serde::Serialize)]
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
        Ok(AchievementLootList_2 { _0, _1, _2, _3 })
    }
}
