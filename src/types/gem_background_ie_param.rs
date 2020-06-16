use crate::BinaryData;
use crate::decode::{Decode, DecodeState};
use std::io;
use std::marker::PhantomData;

#[derive(Debug, Clone, serde::Serialize)]
pub struct GemBackgroundIeParam {
    pub id: i32,
    pub background_feedback: i32,
}

impl BinaryData for GemBackgroundIeParam {
    fn id(_phantom: PhantomData<Self>) -> i32 {
        30
    }
}

impl Decode for GemBackgroundIeParam {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        let background_feedback = state.decode()?;
        Ok(GemBackgroundIeParam {
            id,
            background_feedback,
        })
    }
}
