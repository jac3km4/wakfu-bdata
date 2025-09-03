use crate::BinaryData;
use crate::decode::{Decode, DecodeState};
use std::io;
use std::marker::PhantomData;

#[derive(Debug, Clone, serde::Serialize)]
pub struct MonsterGroupTemplateTableHw {
    pub _0: i32,
    pub _1: Vec<i32>,
}

impl BinaryData for MonsterGroupTemplateTableHw {
    fn id(_phantom: PhantomData<Self>) -> i32 {
        127
    }
}

impl Decode for MonsterGroupTemplateTableHw {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let _0 = state.decode()?;
        let _1 = state.decode()?;
        Ok(MonsterGroupTemplateTableHw { _0, _1 })
    }
}
