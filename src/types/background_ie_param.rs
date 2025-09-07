use std::io;

use serde::Serialize;

use crate::data::BinaryData;
use crate::decode::{Decode, DecodeState};

#[derive(Debug, Clone, Serialize)]
pub struct BackgroundIeParam {
    pub id: i32,
    pub visual_id: i32,
    pub background_feedback: i32,
    pub _3: BackgroundIeParam_3,
}

impl Decode for BackgroundIeParam {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        let visual_id = state.decode()?;
        let background_feedback = state.decode()?;
        let _3 = state.decode()?;
        Ok(Self {
            id,
            visual_id,
            background_feedback,
            _3,
        })
    }
}

impl BinaryData for BackgroundIeParam {
    const TYPE_ID: i16 = 7;
}

#[derive(Debug, Clone, Serialize)]
pub struct BackgroundIeParam_3 {
    pub _0: i8,
    pub _1: i32,
}

impl Decode for BackgroundIeParam_3 {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let _0 = state.decode()?;
        let _1 = state.decode()?;
        Ok(Self { _0, _1 })
    }
}
