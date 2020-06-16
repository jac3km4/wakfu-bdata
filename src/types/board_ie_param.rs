use crate::BinaryData;
use crate::decode::{Decode, DecodeState};
use std::io;
use std::marker::PhantomData;

#[derive(Debug, Clone, serde::Serialize)]
pub struct BoardIeParam {
    pub id: i32,
}

impl BinaryData for BoardIeParam {
    fn id(_phantom: PhantomData<Self>) -> i32 {
        79
    }
}

impl Decode for BoardIeParam {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        Ok(BoardIeParam { id })
    }
}
