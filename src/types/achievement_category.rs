use crate::BinaryData;
use crate::decode::{Decode, DecodeState};
use std::io;
use std::marker::PhantomData;

#[derive(Debug, Clone, serde::Serialize)]
pub struct AchievementCategory {
    pub id: i32,
    pub parent_id: i32,
    pub _2: String,
}

impl BinaryData for AchievementCategory {
    fn id(_phantom: PhantomData<Self>) -> i32 {
        3
    }
}

impl Decode for AchievementCategory {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        let parent_id = state.decode()?;
        let _2 = state.decode()?;
        Ok(AchievementCategory { id, parent_id, _2 })
    }
}
