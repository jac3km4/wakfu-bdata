use crate::BinaryData;
use crate::decode::{Decode, DecodeState};
use std::io;
use std::marker::PhantomData;

#[derive(Debug, Clone, serde::Serialize)]
pub struct DisassemblingDropRate {
    pub _0: i16,
    pub _1: f64,
}

impl BinaryData for DisassemblingDropRate {
    fn id(_phantom: PhantomData<Self>) -> i32 {
        84
    }
}

impl Decode for DisassemblingDropRate {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let _0 = state.decode()?;
        let _1 = state.decode()?;
        Ok(DisassemblingDropRate { _0, _1 })
    }
}
