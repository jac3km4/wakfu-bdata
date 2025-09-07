use std::io;

use serde::Serialize;

use crate::data::BinaryData;
use crate::decode::{Decode, DecodeState};

#[derive(Debug, Clone, Serialize)]
pub struct ClimateBonusList {
    pub buff_list_id: i32,
    pub entries: Vec<i32>,
}

impl Decode for ClimateBonusList {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let buff_list_id = state.decode()?;
        let entries = state.decode()?;
        Ok(Self {
            buff_list_id,
            entries,
        })
    }
}

impl BinaryData for ClimateBonusList {
    const TYPE_ID: i16 = 21;
}
