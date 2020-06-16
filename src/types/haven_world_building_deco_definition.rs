use crate::BinaryData;
use crate::decode::{Decode, DecodeState};
use std::io;
use std::marker::PhantomData;

#[derive(Debug, Clone, serde::Serialize)]
pub struct HavenWorldBuildingDecoDefinition {
    pub id: i32,
    pub catalog_entry_id: i16,
    pub kama_cost: i32,
    pub ressource_cost: i32,
    pub editor_group_id: i32,
}

impl BinaryData for HavenWorldBuildingDecoDefinition {
    fn id(_phantom: PhantomData<Self>) -> i32 {
        126
    }
}

impl Decode for HavenWorldBuildingDecoDefinition {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        let catalog_entry_id = state.decode()?;
        let kama_cost = state.decode()?;
        let ressource_cost = state.decode()?;
        let editor_group_id = state.decode()?;
        Ok(HavenWorldBuildingDecoDefinition {
            id,
            catalog_entry_id,
            kama_cost,
            ressource_cost,
            editor_group_id,
        })
    }
}
