use crate::BinaryData;
use crate::decode::{Decode, DecodeState};
use std::io;
use std::marker::PhantomData;

#[derive(Debug, Clone, serde::Serialize)]
pub struct ExchangeIeParam {
    pub id: i32,
    pub aps_id: i32,
    pub visual_mru_id: i32,
    pub sort_type_id: i8,
    pub _4: Vec<ExchangeIeParam_4>,
    pub _5: ExchangeIeParam_5,
}

impl BinaryData for ExchangeIeParam {
    fn id(_phantom: PhantomData<Self>) -> i32 {
        82
    }
}

impl Decode for ExchangeIeParam {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        let aps_id = state.decode()?;
        let visual_mru_id = state.decode()?;
        let sort_type_id = state.decode()?;
        let _4 = state.decode()?;
        let _5 = state.decode()?;
        Ok(ExchangeIeParam {
            id,
            aps_id,
            visual_mru_id,
            sort_type_id,
            _4,
            _5,
        })
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct ExchangeIeParam_5 {
    pub _0: i8,
    pub _1: i32,
}

impl Decode for ExchangeIeParam_5 {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let _0 = state.decode()?;
        let _1 = state.decode()?;
        Ok(ExchangeIeParam_5 { _0, _1 })
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct ExchangeIeParam_4_5 {
    pub _0: i32,
    pub _1: i16,
    pub _2: i8,
    pub _3: bool,
}

impl Decode for ExchangeIeParam_4_5 {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let _0 = state.decode()?;
        let _1 = state.decode()?;
        let _2 = state.decode()?;
        let _3 = state.decode()?;
        Ok(ExchangeIeParam_4_5 { _0, _1, _2, _3 })
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct ExchangeIeParam_4_2 {
    pub _0: i32,
    pub _1: i16,
}

impl Decode for ExchangeIeParam_4_2 {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let _0 = state.decode()?;
        let _1 = state.decode()?;
        Ok(ExchangeIeParam_4_2 { _0, _1 })
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct ExchangeIeParam_4 {
    pub _0: i32,
    pub _1: String,
    pub _2: Vec<ExchangeIeParam_4_2>,
    pub _3: i32,
    pub _4: i32,
    pub _5: Vec<ExchangeIeParam_4_5>,
    pub _6: i32,
}

impl Decode for ExchangeIeParam_4 {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let _0 = state.decode()?;
        let _1 = state.decode()?;
        let _2 = state.decode()?;
        let _3 = state.decode()?;
        let _4 = state.decode()?;
        let _5 = state.decode()?;
        let _6 = state.decode()?;
        Ok(ExchangeIeParam_4 {
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
