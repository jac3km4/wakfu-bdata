use std::io;

use serde::Serialize;

use crate::data::BinaryData;
use crate::decode::{Decode, DecodeState};

#[derive(Debug, Clone, Serialize)]
pub struct AchievementCategory {
    pub id: i32,
    pub parent_id: i32,
    pub _2: String,
}

impl Decode for AchievementCategory {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        let parent_id = state.decode()?;
        let _2 = state.decode()?;
        Ok(Self { id, parent_id, _2 })
    }
}

impl BinaryData for AchievementCategory {
    const TYPE_ID: i16 = 3;
}
