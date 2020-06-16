use crate::BinaryData;
use crate::decode::{Decode, DecodeState};
use std::io;
use std::marker::PhantomData;

#[derive(Debug, Clone, serde::Serialize)]
pub struct BookcaseIeParam {
    pub id: i32,
    pub size: i8,
    pub _2: Vec<i32>,
}

impl BinaryData for BookcaseIeParam {
    fn id(_phantom: PhantomData<Self>) -> i32 {
        105
    }
}

impl Decode for BookcaseIeParam {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        let size = state.decode()?;
        let _2 = state.decode()?;
        Ok(BookcaseIeParam { id, size, _2 })
    }
}
