use std::io;

use serde::Serialize;

use crate::data::BinaryData;
use crate::decode::{Decode, DecodeState};

#[derive(Debug, Clone, Serialize)]
pub struct GemBackgroundIeParam {
    pub id: i32,
    pub background_feedback: i32,
}

impl Decode for GemBackgroundIeParam {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        let background_feedback = state.decode()?;
        Ok(Self {
            id,
            background_feedback,
        })
    }
}

impl BinaryData for GemBackgroundIeParam {
    const TYPE_ID: i16 = 30;
}
