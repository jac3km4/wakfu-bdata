use std::io;

use serde::Serialize;

use crate::data::BinaryData;
use crate::decode::{Decode, DecodeState};

#[derive(Debug, Clone, Serialize)]
pub struct FurnitureType {
    pub _0: i32,
    pub _1: i32,
}

impl Decode for FurnitureType {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let _0 = state.decode()?;
        let _1 = state.decode()?;
        Ok(Self { _0, _1 })
    }
}

impl BinaryData for FurnitureType {
    const TYPE_ID: i16 = 153;
}
