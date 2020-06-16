use crate::BinaryData;
use crate::decode::{Decode, DecodeState};
use std::io;
use std::marker::PhantomData;

#[derive(Debug, Clone, serde::Serialize)]
pub struct BonusPointDistributionTable {
    pub _0: i16,
    pub _1: std::collections::HashMap<i32, Vec<BonusPointDistributionTable_1>>,
}

impl BinaryData for BonusPointDistributionTable {
    fn id(_phantom: PhantomData<Self>) -> i32 {
        11
    }
}

impl Decode for BonusPointDistributionTable {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let _0 = state.decode()?;
        let _1 = state.decode()?;
        Ok(BonusPointDistributionTable { _0, _1 })
    }
}

#[derive(Debug, Clone, serde::Serialize)]
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
        Ok(BonusPointDistributionTable_1 { _0, _1, _2 })
    }
}
