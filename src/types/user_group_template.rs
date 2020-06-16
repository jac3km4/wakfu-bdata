use crate::BinaryData;
use crate::decode::{Decode, DecodeState};
use std::io;
use std::marker::PhantomData;

#[derive(Debug, Clone, serde::Serialize)]
pub struct UserGroupTemplate {
    pub _0: i32,
    pub _1: bool,
    pub _2: std::collections::HashMap<i32, UserGroupTemplate_2>,
}

impl BinaryData for UserGroupTemplate {
    fn id(_phantom: PhantomData<Self>) -> i32 {
        74
    }
}

impl Decode for UserGroupTemplate {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let _0 = state.decode()?;
        let _1 = state.decode()?;
        let _2 = state.decode()?;
        Ok(UserGroupTemplate { _0, _1, _2 })
    }
}

#[derive(Debug, Clone, serde::Serialize)]
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
        Ok(UserGroupTemplate_2 { _0, _1, _2 })
    }
}
