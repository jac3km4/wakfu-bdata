use crate::BinaryData;
use crate::decode::{Decode, DecodeState};
use std::io;
use std::marker::PhantomData;

#[derive(Debug, Clone, serde::Serialize)]
pub struct Tutorial {
    pub id: i32,
    pub event_ids: Vec<TutorialEventIds>,
}

impl BinaryData for Tutorial {
    fn id(_phantom: PhantomData<Self>) -> i32 {
        129
    }
}

impl Decode for Tutorial {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        let event_ids = state.decode()?;
        Ok(Tutorial { id, event_ids })
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct TutorialEventIds {
    pub event_id: i32,
}

impl Decode for TutorialEventIds {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let event_id = state.decode()?;
        Ok(TutorialEventIds { event_id })
    }
}
