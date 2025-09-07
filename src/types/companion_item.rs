use std::io;

use serde::Serialize;

use crate::data::BinaryData;
use crate::decode::{Decode, DecodeState};

#[derive(Debug, Clone, Serialize)]
pub struct CompanionItem {
    pub id: i32,
}

impl Decode for CompanionItem {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        Ok(Self { id })
    }
}

impl BinaryData for CompanionItem {
    const TYPE_ID: i16 = 131;
}
