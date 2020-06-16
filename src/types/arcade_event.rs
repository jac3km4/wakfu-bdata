use crate::BinaryData;
use crate::decode::{Decode, DecodeState};
use std::io;
use std::marker::PhantomData;

#[derive(Debug, Clone, serde::Serialize)]
pub struct ArcadeEvent {
    pub _0: i32,
    pub _1: Vec<i32>,
    pub _2: Vec<i32>,
    pub _3: i16,
    pub _4: String,
    pub _5: i16,
    pub _6: String,
}

impl BinaryData for ArcadeEvent {
    fn id(_phantom: PhantomData<Self>) -> i32 {
        88
    }
}

impl Decode for ArcadeEvent {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let _0 = state.decode()?;
        let _1 = state.decode()?;
        let _2 = state.decode()?;
        let _3 = state.decode()?;
        let _4 = state.decode()?;
        let _5 = state.decode()?;
        let _6 = state.decode()?;
        Ok(ArcadeEvent {
            _0,
            _1,
            _2,
            _3,
            _4,
            _5,
            _6,
        })
    }
}
