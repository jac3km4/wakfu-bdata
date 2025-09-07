use std::io;

use serde::Serialize;

use crate::data::BinaryData;
use crate::decode::{Decode, DecodeState};

#[derive(Debug, Clone, Serialize)]
pub struct CharacGainPerLevel {
    pub breed_id: i16,
    pub gains: std::collections::HashMap<i8, f32>,
}

impl Decode for CharacGainPerLevel {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let breed_id = state.decode()?;
        let gains = state.decode()?;
        Ok(Self { breed_id, gains })
    }
}

impl BinaryData for CharacGainPerLevel {
    const TYPE_ID: i16 = 16;
}
