use std::io;

use serde::Serialize;

use crate::data::BinaryData;
use crate::decode::{Decode, DecodeState};

#[derive(Debug, Clone, Serialize)]
pub struct AlmanachDate {
    pub id: i32,
    pub date: i64,
    pub almanach_entry_id: i32,
}

impl Decode for AlmanachDate {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        let date = state.decode()?;
        let almanach_entry_id = state.decode()?;
        Ok(Self {
            id,
            date,
            almanach_entry_id,
        })
    }
}

impl BinaryData for AlmanachDate {
    const TYPE_ID: i16 = 99;
}
