use crate::BinaryData;
use crate::decode::{Decode, DecodeState};
use std::io;
use std::marker::PhantomData;

#[derive(Debug, Clone, serde::Serialize)]
pub struct DisassemblingDrops {
    pub _0: i16,
    pub _1: i32,
}

impl BinaryData for DisassemblingDrops {
    fn id(_phantom: PhantomData<Self>) -> i32 {
        83
    }
}

impl Decode for DisassemblingDrops {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let _0 = state.decode()?;
        let _1 = state.decode()?;
        Ok(DisassemblingDrops { _0, _1 })
    }
}
