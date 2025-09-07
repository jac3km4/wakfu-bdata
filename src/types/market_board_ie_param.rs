use std::io;

use serde::Serialize;

use crate::data::BinaryData;
use crate::decode::{Decode, DecodeState};

#[derive(Debug, Clone, Serialize)]
pub struct MarketBoardIeParam {
    pub id: i32,
    pub visual_mru_id: i32,
    pub market_id: i32,
    pub _3: MarketBoardIeParam_3,
}

impl Decode for MarketBoardIeParam {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        let visual_mru_id = state.decode()?;
        let market_id = state.decode()?;
        let _3 = state.decode()?;
        Ok(Self {
            id,
            visual_mru_id,
            market_id,
            _3,
        })
    }
}

impl BinaryData for MarketBoardIeParam {
    const TYPE_ID: i16 = 41;
}

#[derive(Debug, Clone, Serialize)]
pub struct MarketBoardIeParam_3 {
    pub _0: i8,
    pub _1: i32,
}

impl Decode for MarketBoardIeParam_3 {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let _0 = state.decode()?;
        let _1 = state.decode()?;
        Ok(Self { _0, _1 })
    }
}
