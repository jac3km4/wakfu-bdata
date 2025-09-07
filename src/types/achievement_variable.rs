use std::io;

use serde::Serialize;

use crate::data::BinaryData;
use crate::decode::{Decode, DecodeState};

#[derive(Debug, Clone, Serialize)]
pub struct AchievementVariable {
    pub id: i32,
    pub name: String,
    pub export_for_steam: bool,
}

impl Decode for AchievementVariable {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        let name = state.decode()?;
        let export_for_steam = state.decode()?;
        Ok(Self {
            id,
            name,
            export_for_steam,
        })
    }
}

impl BinaryData for AchievementVariable {
    const TYPE_ID: i16 = 2;
}
