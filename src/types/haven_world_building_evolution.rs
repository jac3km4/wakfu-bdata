use std::io;

use serde::Serialize;

use crate::data::BinaryData;
use crate::decode::{Decode, DecodeState};

#[derive(Debug, Clone, Serialize)]
pub struct HavenWorldBuildingEvolution {
    pub id: i32,
    pub catalog_entry_id: i16,
    pub from_id: i32,
    pub to_id: i32,
    pub delay: i64,
    pub order: i8,
}

impl Decode for HavenWorldBuildingEvolution {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        let catalog_entry_id = state.decode()?;
        let from_id = state.decode()?;
        let to_id = state.decode()?;
        let delay = state.decode()?;
        let order = state.decode()?;
        Ok(Self {
            id,
            catalog_entry_id,
            from_id,
            to_id,
            delay,
            order,
        })
    }
}

impl BinaryData for HavenWorldBuildingEvolution {
    const TYPE_ID: i16 = 103;
}
