use crate::BinaryData;
use crate::decode::{Decode, DecodeState};
use std::io;
use std::marker::PhantomData;

#[derive(Debug, Clone, serde::Serialize)]
pub struct StatueIeParam {
    pub _0: i32,
}

impl BinaryData for StatueIeParam {
    fn id(_phantom: PhantomData<Self>) -> i32 {
        93
    }
}

impl Decode for StatueIeParam {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let _0 = state.decode()?;
        Ok(StatueIeParam { _0 })
    }
}
