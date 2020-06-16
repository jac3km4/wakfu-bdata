use crate::BinaryData;
use crate::decode::{Decode, DecodeState};
use std::io;
use std::marker::PhantomData;

#[derive(Debug, Clone, serde::Serialize)]
pub struct DecorationIeParam {
    pub id: i32,
    pub havre_gem_types: Vec<i32>,
}

impl BinaryData for DecorationIeParam {
    fn id(_phantom: PhantomData<Self>) -> i32 {
        25
    }
}

impl Decode for DecorationIeParam {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        let havre_gem_types = state.decode()?;
        Ok(DecorationIeParam { id, havre_gem_types })
    }
}
