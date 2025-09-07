use std::io;

use serde::Serialize;

use crate::data::BinaryData;
use crate::decode::{Decode, DecodeState};

#[derive(Debug, Clone, Serialize)]
pub struct MonsterType {
    pub id: i32,
    pub parent_id: i32,
    pub kind: i8,
}

impl Decode for MonsterType {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        let parent_id = state.decode()?;
        let kind = state.decode()?;
        Ok(Self {
            id,
            parent_id,
            kind,
        })
    }
}

impl BinaryData for MonsterType {
    const TYPE_ID: i16 = 47;
}
