use crate::BinaryData;
use crate::decode::{Decode, DecodeState};
use std::io;
use std::marker::PhantomData;

#[derive(Debug, Clone, serde::Serialize)]
pub struct ZaapCategory {
    pub _0: i32,
    pub _1: i32,
}

impl BinaryData for ZaapCategory {
    fn id(_phantom: PhantomData<Self>) -> i32 {
        139
    }
}

impl Decode for ZaapCategory {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let _0 = state.decode()?;
        let _1 = state.decode()?;
        Ok(ZaapCategory { _0, _1 })
    }
}
