use crate::BinaryData;
use crate::decode::{Decode, DecodeState};
use std::io;
use std::marker::PhantomData;

#[derive(Debug, Clone, serde::Serialize)]
pub struct AchievementVariable {
    pub id: i32,
    pub name: String,
    pub export_for_steam: bool,
}

impl BinaryData for AchievementVariable {
    fn id(_phantom: PhantomData<Self>) -> i32 {
        2
    }
}

impl Decode for AchievementVariable {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        let name = state.decode()?;
        let export_for_steam = state.decode()?;
        Ok(AchievementVariable {
            id,
            name,
            export_for_steam,
        })
    }
}
