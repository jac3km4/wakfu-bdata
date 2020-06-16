use crate::BinaryData;
use crate::decode::{Decode, DecodeState};
use std::io;
use std::marker::PhantomData;

#[derive(Debug, Clone, serde::Serialize)]
pub struct MagicraftLootList {
    pub id: i32,
    pub gem_type: i8,
    pub entries: Vec<MagicraftLootListEntries>,
}

impl BinaryData for MagicraftLootList {
    fn id(_phantom: PhantomData<Self>) -> i32 {
        123
    }
}

impl Decode for MagicraftLootList {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        let gem_type = state.decode()?;
        let entries = state.decode()?;
        Ok(MagicraftLootList { id, gem_type, entries })
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct MagicraftLootListEntries {
    pub item_id: i32,
    pub drop_rate: f64,
}

impl Decode for MagicraftLootListEntries {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let item_id = state.decode()?;
        let drop_rate = state.decode()?;
        Ok(MagicraftLootListEntries { item_id, drop_rate })
    }
}
