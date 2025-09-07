use std::io;

use serde::Serialize;

use crate::data::BinaryData;
use crate::decode::{Decode, DecodeState};

#[derive(Debug, Clone, Serialize)]
pub struct UserGroupTemplate {
    pub _0: i32,
    pub _1: bool,
    pub _2: std::collections::HashMap<i32, UserGroupTemplate_2>,
}

impl Decode for UserGroupTemplate {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let _0 = state.decode()?;
        let _1 = state.decode()?;
        let _2 = state.decode()?;
        Ok(Self { _0, _1, _2 })
    }
}

impl BinaryData for UserGroupTemplate {
    const TYPE_ID: i16 = 74;
}

#[derive(Debug, Clone, Serialize)]
pub struct UserGroupTemplate_2 {
    pub _0: i32,
    pub _1: i16,
    pub _2: String,
}

impl Decode for UserGroupTemplate_2 {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let _0 = state.decode()?;
        let _1 = state.decode()?;
        let _2 = state.decode()?;
        Ok(Self { _0, _1, _2 })
    }
}
