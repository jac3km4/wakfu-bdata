use crate::BinaryData;
use crate::decode::{Decode, DecodeState};
use std::io;
use std::marker::PhantomData;

#[derive(Debug, Clone, serde::Serialize)]
pub struct MonsterGroupTemplateTable {
    pub _0: i32,
    pub _1: i32,
    pub _2: std::collections::HashMap<i32, f32>,
}

impl BinaryData for MonsterGroupTemplateTable {
    fn id(_phantom: PhantomData<Self>) -> i32 {
        46
    }
}

impl Decode for MonsterGroupTemplateTable {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let _0 = state.decode()?;
        let _1 = state.decode()?;
        let _2 = state.decode()?;
        Ok(MonsterGroupTemplateTable { _0, _1, _2 })
    }
}
