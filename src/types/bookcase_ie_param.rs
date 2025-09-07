use std::io;

use serde::Serialize;

use crate::data::BinaryData;
use crate::decode::{Decode, DecodeState};

#[derive(Debug, Clone, Serialize)]
pub struct BookcaseIeParam {
    pub id: i32,
    pub size: i8,
    pub _2: Vec<i32>,
}

impl Decode for BookcaseIeParam {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        let size = state.decode()?;
        let _2 = state.decode()?;
        Ok(Self { id, size, _2 })
    }
}

impl BinaryData for BookcaseIeParam {
    const TYPE_ID: i16 = 105;
}
