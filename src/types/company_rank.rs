use crate::BinaryData;
use crate::decode::{Decode, DecodeState};
use std::io;
use std::marker::PhantomData;

#[derive(Debug, Clone, serde::Serialize)]
pub struct CompanyRank {
    pub _0: i32,
    pub _1: i16,
    pub _2: i16,
    pub _3: i16,
    pub _4: i16,
}

impl BinaryData for CompanyRank {
    fn id(_phantom: PhantomData<Self>) -> i32 {
        138
    }
}

impl Decode for CompanyRank {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let _0 = state.decode()?;
        let _1 = state.decode()?;
        let _2 = state.decode()?;
        let _3 = state.decode()?;
        let _4 = state.decode()?;
        Ok(CompanyRank { _0, _1, _2, _3, _4 })
    }
}
