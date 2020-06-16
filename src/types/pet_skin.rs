use crate::decode::{Decode, DecodeState};
use crate::BinaryData;
use std::io;
use std::marker::PhantomData;

#[derive(Debug, Clone, serde::Serialize)]
pub struct PetSkin {
    pub _0: i32,
    pub _1: i32,
    pub _2: i32,
    pub _3: i32,
}

impl BinaryData for PetSkin {
    fn id(_phantom: PhantomData<Self>) -> i32 {
        146
    }
}

impl Decode for PetSkin {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let _0 = state.decode()?;
        let _1 = state.decode()?;
        let _2 = state.decode()?;
        let _3 = state.decode()?;
        Ok(PetSkin{ _0, _1, _2, _3 })
    }
}

