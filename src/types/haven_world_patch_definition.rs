use std::io;

use serde::Serialize;

use crate::data::BinaryData;
use crate::decode::{Decode, DecodeState};

#[derive(Debug, Clone, Serialize)]
pub struct HavenWorldPatchDefinition {
    pub id: i32,
    pub patch_id: i32,
    pub kama_cost: i32,
    pub category_id: i32,
    pub sound_id: i32,
}

impl Decode for HavenWorldPatchDefinition {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        let patch_id = state.decode()?;
        let kama_cost = state.decode()?;
        let category_id = state.decode()?;
        let sound_id = state.decode()?;
        Ok(Self {
            id,
            patch_id,
            kama_cost,
            category_id,
            sound_id,
        })
    }
}

impl BinaryData for HavenWorldPatchDefinition {
    const TYPE_ID: i16 = 101;
}
