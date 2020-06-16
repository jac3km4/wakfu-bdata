use crate::BinaryData;
use crate::decode::{Decode, DecodeState};
use std::io;
use std::marker::PhantomData;

#[derive(Debug, Clone, serde::Serialize)]
pub struct ZaapLink {
    pub id: i32,
    pub start: i32,
    pub end: i32,
}

impl BinaryData for ZaapLink {
    fn id(_phantom: PhantomData<Self>) -> i32 {
        77
    }
}

impl Decode for ZaapLink {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        let start = state.decode()?;
        let end = state.decode()?;
        Ok(ZaapLink { id, start, end })
    }
}
