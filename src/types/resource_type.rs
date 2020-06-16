use crate::BinaryData;
use crate::decode::{Decode, DecodeState};
use std::io;
use std::marker::PhantomData;

#[derive(Debug, Clone, serde::Serialize)]
pub struct ResourceType {
    pub _0: i32,
    pub _1: bool,
}

impl BinaryData for ResourceType {
    fn id(_phantom: PhantomData<Self>) -> i32 {
        61
    }
}

impl Decode for ResourceType {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let _0 = state.decode()?;
        let _1 = state.decode()?;
        Ok(ResourceType { _0, _1 })
    }
}
