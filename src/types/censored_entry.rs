use std::io;

use serde::Serialize;

use crate::data::BinaryData;
use crate::decode::{Decode, DecodeState};

#[derive(Debug, Clone, Serialize)]
pub struct CensoredEntry {
    pub id: i32,
    pub deep_search: bool,
    pub language: i32,
    pub censor_type: i32,
    pub text: String,
}

impl Decode for CensoredEntry {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        let deep_search = state.decode()?;
        let language = state.decode()?;
        let censor_type = state.decode()?;
        let text = state.decode()?;
        Ok(Self {
            id,
            deep_search,
            language,
            censor_type,
            text,
        })
    }
}

impl BinaryData for CensoredEntry {
    const TYPE_ID: i16 = 13;
}
