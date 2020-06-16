use crate::BinaryData;
use crate::decode::{Decode, DecodeState};
use std::io;
use std::marker::PhantomData;

#[derive(Debug, Clone, serde::Serialize)]
pub struct AudioMarkerIeParam {
    pub id: i32,
    pub audio_marker_type_id: i32,
    pub is_localized: bool,
}

impl BinaryData for AudioMarkerIeParam {
    fn id(_phantom: PhantomData<Self>) -> i32 {
        5
    }
}

impl Decode for AudioMarkerIeParam {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        let audio_marker_type_id = state.decode()?;
        let is_localized = state.decode()?;
        Ok(AudioMarkerIeParam {
            id,
            audio_marker_type_id,
            is_localized,
        })
    }
}
