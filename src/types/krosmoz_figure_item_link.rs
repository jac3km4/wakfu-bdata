use crate::BinaryData;
use crate::decode::{Decode, DecodeState};
use std::io;
use std::marker::PhantomData;

#[derive(Debug, Clone, serde::Serialize)]
pub struct KrosmozFigureItemLink {
    pub _0: i32,
    pub _1: Vec<String>,
}

impl BinaryData for KrosmozFigureItemLink {
    fn id(_phantom: PhantomData<Self>) -> i32 {
        112
    }
}

impl Decode for KrosmozFigureItemLink {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let _0 = state.decode()?;
        let _1 = state.decode()?;
        Ok(KrosmozFigureItemLink { _0, _1 })
    }
}
