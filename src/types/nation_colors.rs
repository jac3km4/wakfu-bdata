use std::io;

use serde::Serialize;

use crate::data::BinaryData;
use crate::decode::{Decode, DecodeState};

#[derive(Debug, Clone, Serialize)]
pub struct NationColors {
    pub id: i32,
    pub color: String,
}

impl Decode for NationColors {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        let color = state.decode()?;
        Ok(Self { id, color })
    }
}

impl BinaryData for NationColors {
    const TYPE_ID: i16 = 51;
}
