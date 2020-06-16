use crate::BinaryData;
use crate::decode::{Decode, DecodeState};
use std::io;
use std::marker::PhantomData;

#[derive(Debug, Clone, serde::Serialize)]
pub struct DungeonDisplayerIeParam {
    pub id: i32,
    pub dungeon_id: i32,
}

impl BinaryData for DungeonDisplayerIeParam {
    fn id(_phantom: PhantomData<Self>) -> i32 {
        95
    }
}

impl Decode for DungeonDisplayerIeParam {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        let dungeon_id = state.decode()?;
        Ok(DungeonDisplayerIeParam { id, dungeon_id })
    }
}
