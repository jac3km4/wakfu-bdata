use std::io;

use serde::Serialize;

use crate::data::BinaryData;
use crate::decode::{Decode, DecodeState};

#[derive(Debug, Clone, Serialize)]
pub struct HavenWorldBuildingDecoDefinition {
    pub id: i32,
    pub catalog_entry_id: i16,
    pub kama_cost: i32,
    pub ressource_cost: i32,
    pub editor_group_id: i32,
}

impl Decode for HavenWorldBuildingDecoDefinition {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        let catalog_entry_id = state.decode()?;
        let kama_cost = state.decode()?;
        let ressource_cost = state.decode()?;
        let editor_group_id = state.decode()?;
        Ok(Self {
            id,
            catalog_entry_id,
            kama_cost,
            ressource_cost,
            editor_group_id,
        })
    }
}

impl BinaryData for HavenWorldBuildingDecoDefinition {
    const TYPE_ID: i16 = 126;
}
