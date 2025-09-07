use std::io;

use serde::Serialize;

use crate::data::BinaryData;
use crate::decode::{Decode, DecodeState};

#[derive(Debug, Clone, Serialize)]
pub struct Secret {
    pub id: i32,
    pub level: i16,
    pub item_id: i16,
}

impl Decode for Secret {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        let level = state.decode()?;
        let item_id = state.decode()?;
        Ok(Self { id, level, item_id })
    }
}

impl BinaryData for Secret {
    const TYPE_ID: i16 = 133;
}
