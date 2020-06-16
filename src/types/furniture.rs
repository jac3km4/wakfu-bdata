use crate::decode::{Decode, DecodeState};
use crate::BinaryData;
use std::io;
use std::marker::PhantomData;

#[derive(Debug, Clone, serde::Serialize)]
pub struct Furniture {
    pub _0: i32,
    pub _1: i32,
}

impl BinaryData for Furniture {
    fn id(_phantom: PhantomData<Self>) -> i32 {
        152
    }
}

impl Decode for Furniture {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let _0 = state.decode()?;
        let _1 = state.decode()?;
        Ok(Furniture{ _0, _1 })
    }
}

