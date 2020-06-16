use crate::BinaryData;
use crate::decode::{Decode, DecodeState};
use std::io;
use std::marker::PhantomData;

#[derive(Debug, Clone, serde::Serialize)]
pub struct HavenWorldPatchDefinition {
    pub id: i32,
    pub patch_id: i32,
    pub kama_cost: i32,
    pub category_id: i32,
    pub sound_id: i32,
}

impl BinaryData for HavenWorldPatchDefinition {
    fn id(_phantom: PhantomData<Self>) -> i32 {
        101
    }
}

impl Decode for HavenWorldPatchDefinition {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        let patch_id = state.decode()?;
        let kama_cost = state.decode()?;
        let category_id = state.decode()?;
        let sound_id = state.decode()?;
        Ok(HavenWorldPatchDefinition {
            id,
            patch_id,
            kama_cost,
            category_id,
            sound_id,
        })
    }
}
