use std::io;

use serde::Serialize;

use crate::data::BinaryData;
use crate::decode::{Decode, DecodeState};

#[derive(Debug, Clone, Serialize)]
pub struct AudioMarkerIeParam {
    pub id: i32,
    pub audio_marker_type_id: i32,
    pub is_localized: bool,
}

impl Decode for AudioMarkerIeParam {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        let audio_marker_type_id = state.decode()?;
        let is_localized = state.decode()?;
        Ok(Self {
            id,
            audio_marker_type_id,
            is_localized,
        })
    }
}

impl BinaryData for AudioMarkerIeParam {
    const TYPE_ID: i16 = 5;
}
