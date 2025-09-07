use std::io;

use serde::Serialize;

use crate::data::BinaryData;
use crate::decode::{Decode, DecodeState};

#[derive(Debug, Clone, Serialize)]
pub struct BonusPointDistributionTable {
    pub _0: i16,
    pub _1: std::collections::HashMap<i32, Vec<BonusPointDistributionTable_1>>,
}

impl Decode for BonusPointDistributionTable {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let _0 = state.decode()?;
        let _1 = state.decode()?;
        Ok(Self { _0, _1 })
    }
}

impl BinaryData for BonusPointDistributionTable {
    const TYPE_ID: i16 = 11;
}

#[derive(Debug, Clone, Serialize)]
pub struct BonusPointDistributionTable_1 {
    pub _0: i16,
    pub _1: i16,
    pub _2: i16,
}

impl Decode for BonusPointDistributionTable_1 {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let _0 = state.decode()?;
        let _1 = state.decode()?;
        let _2 = state.decode()?;
        Ok(Self { _0, _1, _2 })
    }
}
