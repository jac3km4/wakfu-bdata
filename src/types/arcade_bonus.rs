use crate::BinaryData;
use crate::decode::{Decode, DecodeState};
use std::io;
use std::marker::PhantomData;

#[derive(Debug, Clone, serde::Serialize)]
pub struct ArcadeBonus {
    pub _0: i32,
    pub _1: Vec<i32>,
    pub _2: Vec<i32>,
}

impl BinaryData for ArcadeBonus {
    fn id(_phantom: PhantomData<Self>) -> i32 {
        89
    }
}

impl Decode for ArcadeBonus {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let _0 = state.decode()?;
        let _1 = state.decode()?;
        let _2 = state.decode()?;
        Ok(ArcadeBonus { _0, _1, _2 })
    }
}
