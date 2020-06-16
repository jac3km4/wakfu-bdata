use crate::BinaryData;
use crate::decode::{Decode, DecodeState};
use std::io;
use std::marker::PhantomData;

#[derive(Debug, Clone, serde::Serialize)]
pub struct LadderBoardIeParam {
    pub _0: i32,
}

impl BinaryData for LadderBoardIeParam {
    fn id(_phantom: PhantomData<Self>) -> i32 {
        94
    }
}

impl Decode for LadderBoardIeParam {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let _0 = state.decode()?;
        Ok(LadderBoardIeParam { _0 })
    }
}
