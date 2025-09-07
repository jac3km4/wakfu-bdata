use std::io;

use serde::Serialize;

use crate::data::BinaryData;
use crate::decode::{Decode, DecodeState};

#[derive(Debug, Clone, Serialize)]
pub struct ChallengeLootList {
    pub id: i32,
    pub entries: Vec<ChallengeLootListEntries>,
}

impl Decode for ChallengeLootList {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        let entries = state.decode()?;
        Ok(Self { id, entries })
    }
}

impl BinaryData for ChallengeLootList {
    const TYPE_ID: i16 = 80;
}

#[derive(Debug, Clone, Serialize)]
pub struct ChallengeLootListEntries {
    pub challenge_id: i32,
    pub criteria: String,
}

impl Decode for ChallengeLootListEntries {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let challenge_id = state.decode()?;
        let criteria = state.decode()?;
        Ok(Self {
            challenge_id,
            criteria,
        })
    }
}
