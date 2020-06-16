use crate::BinaryData;
use crate::decode::{Decode, DecodeState};
use std::io;
use std::marker::PhantomData;

#[derive(Debug, Clone, serde::Serialize)]
pub struct AchievementList {
    pub id: i32,
    pub elements: Vec<AchievementListElements>,
}

impl BinaryData for AchievementList {
    fn id(_phantom: PhantomData<Self>) -> i32 {
        115
    }
}

impl Decode for AchievementList {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        let elements = state.decode()?;
        Ok(AchievementList { id, elements })
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct AchievementListElements {
    pub achievement_id: i32,
    pub order: i32,
}

impl Decode for AchievementListElements {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let achievement_id = state.decode()?;
        let order = state.decode()?;
        Ok(AchievementListElements { achievement_id, order })
    }
}
