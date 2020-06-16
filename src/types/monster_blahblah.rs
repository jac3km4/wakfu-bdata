use crate::BinaryData;
use crate::decode::{Decode, DecodeState};
use std::io;
use std::marker::PhantomData;

#[derive(Debug, Clone, serde::Serialize)]
pub struct MonsterBlahblah {
    pub _0: i32,
    pub _1: i32,
    pub _2: i32,
    pub _3: i8,
    pub _4: Vec<MonsterBlahblah_4>,
}

impl BinaryData for MonsterBlahblah {
    fn id(_phantom: PhantomData<Self>) -> i32 {
        43
    }
}

impl Decode for MonsterBlahblah {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let _0 = state.decode()?;
        let _1 = state.decode()?;
        let _2 = state.decode()?;
        let _3 = state.decode()?;
        let _4 = state.decode()?;
        Ok(MonsterBlahblah { _0, _1, _2, _3, _4 })
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct MonsterBlahblah_4 {
    pub _0: i32,
    pub _1: i8,
    pub _2: bool,
    pub _3: String,
}

impl Decode for MonsterBlahblah_4 {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let _0 = state.decode()?;
        let _1 = state.decode()?;
        let _2 = state.decode()?;
        let _3 = state.decode()?;
        Ok(MonsterBlahblah_4 { _0, _1, _2, _3 })
    }
}
