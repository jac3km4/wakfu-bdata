use std::io;

use serde::Serialize;

use crate::data::BinaryData;
use crate::decode::{Decode, DecodeState};

#[derive(Debug, Clone, Serialize)]
pub struct MagicraftLootList {
    pub id: i32,
    pub gem_type: i8,
    pub entries: Vec<MagicraftLootListEntries>,
}

impl Decode for MagicraftLootList {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        let gem_type = state.decode()?;
        let entries = state.decode()?;
        Ok(Self {
            id,
            gem_type,
            entries,
        })
    }
}

impl BinaryData for MagicraftLootList {
    const TYPE_ID: i16 = 123;
}

#[derive(Debug, Clone, Serialize)]
pub struct MagicraftLootListEntries {
    pub item_id: i32,
    pub drop_rate: f64,
}

impl Decode for MagicraftLootListEntries {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let item_id = state.decode()?;
        let drop_rate = state.decode()?;
        Ok(Self { item_id, drop_rate })
    }
}
