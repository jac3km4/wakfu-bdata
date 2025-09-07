use std::io;

use serde::Serialize;

use crate::data::BinaryData;
use crate::decode::{Decode, DecodeState};

#[derive(Debug, Clone, Serialize)]
pub struct AlmanachEntry {
    pub id: i32,
    pub scenario_id: i32,
    pub achievement_id: i32,
    pub territories: Vec<i32>,
}

impl Decode for AlmanachEntry {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        let scenario_id = state.decode()?;
        let achievement_id = state.decode()?;
        let territories = state.decode()?;
        Ok(Self {
            id,
            scenario_id,
            achievement_id,
            territories,
        })
    }
}

impl BinaryData for AlmanachEntry {
    const TYPE_ID: i16 = 98;
}
