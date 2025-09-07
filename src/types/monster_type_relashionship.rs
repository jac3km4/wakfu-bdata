use std::io;

use serde::Serialize;

use crate::data::BinaryData;
use crate::decode::{Decode, DecodeState};

#[derive(Debug, Clone, Serialize)]
pub struct MonsterTypeRelashionship {
    pub id: i32,
    pub family_from: i32,
    pub family_to: i32,
}

impl Decode for MonsterTypeRelashionship {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        let family_from = state.decode()?;
        let family_to = state.decode()?;
        Ok(Self {
            id,
            family_from,
            family_to,
        })
    }
}

impl BinaryData for MonsterTypeRelashionship {
    const TYPE_ID: i16 = 49;
}
