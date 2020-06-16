use crate::decode::{Decode, DecodeState};
use crate::BinaryData;
use std::io;
use std::marker::PhantomData;

#[derive(Debug, Clone, serde::Serialize)]
pub struct Guides {
    pub _0: i32,
    pub _1: i16,
    pub _2: i32,
    pub _3: i32,
    pub _4: Vec<Guides_4>,
}

impl BinaryData for Guides {
    fn id(_phantom: PhantomData<Self>) -> i32 {
        150
    }
}

impl Decode for Guides {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let _0 = state.decode()?;
        let _1 = state.decode()?;
        let _2 = state.decode()?;
        let _3 = state.decode()?;
        let _4 = state.decode()?;
        Ok(Guides{ _0, _1, _2, _3, _4 })
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct Guides_4_5 {
    pub _0: i32,
    pub _1: i16,
    pub _2: String,
}

impl Decode for Guides_4_5 {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let _0 = state.decode()?;
        let _1 = state.decode()?;
        let _2 = state.decode()?;
        Ok(Guides_4_5{ _0, _1, _2 })
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct Guides_4 {
    pub _0: i32,
    pub _1: i16,
    pub _2: String,
    pub _3: String,
    pub _4: bool,
    pub _5: Vec<Guides_4_5>,
}

impl Decode for Guides_4 {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let _0 = state.decode()?;
        let _1 = state.decode()?;
        let _2 = state.decode()?;
        let _3 = state.decode()?;
        let _4 = state.decode()?;
        let _5 = state.decode()?;
        Ok(Guides_4{ _0, _1, _2, _3, _4, _5 })
    }
}

