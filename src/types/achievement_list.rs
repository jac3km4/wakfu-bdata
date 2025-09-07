use std::io;

use serde::Serialize;

use crate::data::BinaryData;
use crate::decode::{Decode, DecodeState};

#[derive(Debug, Clone, Serialize)]
pub struct AchievementList {
    pub id: i32,
    pub elements: Vec<AchievementListElements>,
}

impl Decode for AchievementList {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        let elements = state.decode()?;
        Ok(Self { id, elements })
    }
}

impl BinaryData for AchievementList {
    const TYPE_ID: i16 = 115;
}

#[derive(Debug, Clone, Serialize)]
pub struct AchievementListElements {
    pub achievement_id: i32,
    pub order: i32,
}

impl Decode for AchievementListElements {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let achievement_id = state.decode()?;
        let order = state.decode()?;
        Ok(Self {
            achievement_id,
            order,
        })
    }
}
