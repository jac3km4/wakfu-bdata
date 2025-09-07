use std::io;

use serde::Serialize;

use crate::data::BinaryData;
use crate::decode::{Decode, DecodeState};

#[derive(Debug, Clone, Serialize)]
pub struct Ground {
    pub id: i32,
    pub resource_fertility: std::collections::HashMap<i32, i16>,
    pub resource_type_fertility: std::collections::HashMap<i16, i16>,
}

impl Decode for Ground {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        let resource_fertility = state.decode()?;
        let resource_type_fertility = state.decode()?;
        Ok(Self {
            id,
            resource_fertility,
            resource_type_fertility,
        })
    }
}

impl BinaryData for Ground {
    const TYPE_ID: i16 = 32;
}
