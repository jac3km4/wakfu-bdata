use crate::BinaryData;
use crate::decode::{Decode, DecodeState};
use std::io;
use std::marker::PhantomData;

#[derive(Debug, Clone, serde::Serialize)]
pub struct GenericIeParam {
    pub _0: i32,
}

impl BinaryData for GenericIeParam {
    fn id(_phantom: PhantomData<Self>) -> i32 {
        92
    }
}

impl Decode for GenericIeParam {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let _0 = state.decode()?;
        Ok(GenericIeParam { _0 })
    }
}
