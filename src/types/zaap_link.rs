use std::io;

use serde::Serialize;

use crate::data::BinaryData;
use crate::decode::{Decode, DecodeState};

#[derive(Debug, Clone, Serialize)]
pub struct ZaapLink {
    pub id: i32,
    pub start: i32,
    pub end: i32,
}

impl Decode for ZaapLink {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        let start = state.decode()?;
        let end = state.decode()?;
        Ok(Self { id, start, end })
    }
}

impl BinaryData for ZaapLink {
    const TYPE_ID: i16 = 77;
}
