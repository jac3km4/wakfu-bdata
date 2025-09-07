use std::io;

use serde::Serialize;

use crate::data::BinaryData;
use crate::decode::{Decode, DecodeState};

#[derive(Debug, Clone, Serialize)]
pub struct DungeonDisplayerIeParam {
    pub id: i32,
    pub dungeon_id: i32,
}

impl Decode for DungeonDisplayerIeParam {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        let dungeon_id = state.decode()?;
        Ok(Self { id, dungeon_id })
    }
}

impl BinaryData for DungeonDisplayerIeParam {
    const TYPE_ID: i16 = 95;
}
