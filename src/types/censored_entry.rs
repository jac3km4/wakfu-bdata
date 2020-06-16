use crate::BinaryData;
use crate::decode::{Decode, DecodeState};
use std::io;
use std::marker::PhantomData;

#[derive(Debug, Clone, serde::Serialize)]
pub struct CensoredEntry {
    pub id: i32,
    pub deep_search: bool,
    pub language: i32,
    pub censor_type: i32,
    pub text: String,
}

impl BinaryData for CensoredEntry {
    fn id(_phantom: PhantomData<Self>) -> i32 {
        13
    }
}

impl Decode for CensoredEntry {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        let deep_search = state.decode()?;
        let language = state.decode()?;
        let censor_type = state.decode()?;
        let text = state.decode()?;
        Ok(CensoredEntry {
            id,
            deep_search,
            language,
            censor_type,
            text,
        })
    }
}
