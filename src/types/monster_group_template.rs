use crate::BinaryData;
use crate::decode::{Decode, DecodeState};
use std::io;
use std::marker::PhantomData;

#[derive(Debug, Clone, serde::Serialize)]
pub struct MonsterGroupTemplate {
    pub _0: i32,
    pub _1: Vec<i32>,
    pub _2: std::collections::HashMap<i32, i32>,
    pub _3: Vec<i32>,
}

impl BinaryData for MonsterGroupTemplate {
    fn id(_phantom: PhantomData<Self>) -> i32 {
        45
    }
}

impl Decode for MonsterGroupTemplate {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let _0 = state.decode()?;
        let _1 = state.decode()?;
        let _2 = state.decode()?;
        let _3 = state.decode()?;
        Ok(MonsterGroupTemplate { _0, _1, _2, _3 })
    }
}
