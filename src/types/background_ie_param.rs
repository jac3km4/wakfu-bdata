use crate::BinaryData;
use crate::decode::{Decode, DecodeState};
use std::io;
use std::marker::PhantomData;

#[derive(Debug, Clone, serde::Serialize)]
pub struct BackgroundIeParam {
    pub id: i32,
    pub visual_id: i32,
    pub background_feedback: i32,
    pub _3: BackgroundIeParam_3,
}

impl BinaryData for BackgroundIeParam {
    fn id(_phantom: PhantomData<Self>) -> i32 {
        7
    }
}

impl Decode for BackgroundIeParam {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        let visual_id = state.decode()?;
        let background_feedback = state.decode()?;
        let _3 = state.decode()?;
        Ok(BackgroundIeParam {
            id,
            visual_id,
            background_feedback,
            _3,
        })
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct BackgroundIeParam_3 {
    pub _0: i8,
    pub _1: i32,
}

impl Decode for BackgroundIeParam_3 {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let _0 = state.decode()?;
        let _1 = state.decode()?;
        Ok(BackgroundIeParam_3 { _0, _1 })
    }
}
