use std::io;

use serde::Serialize;

use crate::data::BinaryData;
use crate::decode::{Decode, DecodeState};

#[derive(Debug, Clone, Serialize)]
pub struct BoardIeParam {
    pub id: i32,
}

impl Decode for BoardIeParam {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        Ok(Self { id })
    }
}

impl BinaryData for BoardIeParam {
    const TYPE_ID: i16 = 79;
}
