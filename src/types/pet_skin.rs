use std::io;

use serde::Serialize;

use crate::data::BinaryData;
use crate::decode::{Decode, DecodeState};

#[derive(Debug, Clone, Serialize)]
pub struct PetSkin {
    pub _0: i32,
    pub _1: i32,
    pub _2: i32,
    pub _3: i32,
}

impl Decode for PetSkin {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let _0 = state.decode()?;
        let _1 = state.decode()?;
        let _2 = state.decode()?;
        let _3 = state.decode()?;
        Ok(Self { _0, _1, _2, _3 })
    }
}

impl BinaryData for PetSkin {
    const TYPE_ID: i16 = 146;
}
