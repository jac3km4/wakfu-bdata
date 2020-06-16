use crate::decode::{Decode, DecodeState};
use crate::BinaryData;
use std::io;
use std::marker::PhantomData;

#[derive(Debug, Clone, serde::Serialize)]
pub struct ArtisansBookIeParam {
    pub _0: i32,
    pub _1: i32,
    pub _2: i32,
    pub _3: ArtisansBookIeParam_3,
}

impl BinaryData for ArtisansBookIeParam {
    fn id(_phantom: PhantomData<Self>) -> i32 {
        145
    }
}

impl Decode for ArtisansBookIeParam {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let _0 = state.decode()?;
        let _1 = state.decode()?;
        let _2 = state.decode()?;
        let _3 = state.decode()?;
        Ok(ArtisansBookIeParam{ _0, _1, _2, _3 })
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct ArtisansBookIeParam_3 {
    pub _0: i8,
    pub _1: i32,
}

impl Decode for ArtisansBookIeParam_3 {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let _0 = state.decode()?;
        let _1 = state.decode()?;
        Ok(ArtisansBookIeParam_3{ _0, _1 })
    }
}

