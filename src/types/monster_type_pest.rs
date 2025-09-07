use std::io;

use serde::Serialize;

use crate::data::BinaryData;
use crate::decode::{Decode, DecodeState};

#[derive(Debug, Clone, Serialize)]
pub struct MonsterTypePest {
    pub id: i32,
    pub family_id: i32,
    pub pest_monster_id: i32,
}

impl Decode for MonsterTypePest {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        let family_id = state.decode()?;
        let pest_monster_id = state.decode()?;
        Ok(Self {
            id,
            family_id,
            pest_monster_id,
        })
    }
}

impl BinaryData for MonsterTypePest {
    const TYPE_ID: i16 = 48;
}
