use crate::decode::{Decode, DecodeState};
use crate::BinaryData;
use std::io;
use std::marker::PhantomData;

#[derive(Debug, Clone, serde::Serialize)]
pub struct FurnitureType {
    pub _0: i32,
    pub _1: i32,
}

impl BinaryData for FurnitureType {
    fn id(_phantom: PhantomData<Self>) -> i32 {
        153
    }
}

impl Decode for FurnitureType {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let _0 = state.decode()?;
        let _1 = state.decode()?;
        Ok(FurnitureType{ _0, _1 })
    }
}

