use std::io;

use serde::Serialize;

use crate::data::BinaryData;
use crate::decode::{Decode, DecodeState};

#[derive(Debug, Clone, Serialize)]
pub struct Tutorial {
    pub id: i32,
    pub event_ids: Vec<TutorialEventIds>,
}

impl Decode for Tutorial {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        let event_ids = state.decode()?;
        Ok(Self { id, event_ids })
    }
}

impl BinaryData for Tutorial {
    const TYPE_ID: i16 = 129;
}

#[derive(Debug, Clone, Serialize)]
pub struct TutorialEventIds {
    pub event_id: i32,
}

impl Decode for TutorialEventIds {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let event_id = state.decode()?;
        Ok(Self { event_id })
    }
}
