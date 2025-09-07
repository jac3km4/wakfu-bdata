use std::io;

use serde::Serialize;

use crate::data::BinaryData;
use crate::decode::{Decode, DecodeState};

#[derive(Debug, Clone, Serialize)]
pub struct StoolIeParam {
    pub id: i32,
    pub criterion: String,
    pub visual_id: i32,
    pub _3: StoolIeParam_3,
}

impl Decode for StoolIeParam {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        let criterion = state.decode()?;
        let visual_id = state.decode()?;
        let _3 = state.decode()?;
        Ok(Self {
            id,
            criterion,
            visual_id,
            _3,
        })
    }
}

impl BinaryData for StoolIeParam {
    const TYPE_ID: i16 = 69;
}

#[derive(Debug, Clone, Serialize)]
pub struct StoolIeParam_3 {
    pub _0: i8,
    pub _1: i32,
}

impl Decode for StoolIeParam_3 {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let _0 = state.decode()?;
        let _1 = state.decode()?;
        Ok(Self { _0, _1 })
    }
}
