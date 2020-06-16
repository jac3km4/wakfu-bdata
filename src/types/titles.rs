use crate::decode::{Decode, DecodeState};
use crate::BinaryData;
use std::io;
use std::marker::PhantomData;

#[derive(Debug, Clone, serde::Serialize)]
pub struct Titles {
    pub _0: i32,
    pub _1: i16,
}

impl BinaryData for Titles {
    fn id(_phantom: PhantomData<Self>) -> i32 {
        149
    }
}

impl Decode for Titles {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let _0 = state.decode()?;
        let _1 = state.decode()?;
        Ok(Titles{ _0, _1 })
    }
}

