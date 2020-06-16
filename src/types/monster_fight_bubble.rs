use crate::BinaryData;
use crate::decode::{Decode, DecodeState};
use std::io;
use std::marker::PhantomData;

#[derive(Debug, Clone, serde::Serialize)]
pub struct MonsterFightBubble {
    pub _0: i32,
    pub _1: bool,
    pub _2: Vec<MonsterFightBubble_2>,
    pub _3: Vec<MonsterFightBubble_3>,
    pub _4: Vec<MonsterFightBubble_4>,
}

impl BinaryData for MonsterFightBubble {
    fn id(_phantom: PhantomData<Self>) -> i32 {
        44
    }
}

impl Decode for MonsterFightBubble {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let _0 = state.decode()?;
        let _1 = state.decode()?;
        let _2 = state.decode()?;
        let _3 = state.decode()?;
        let _4 = state.decode()?;
        Ok(MonsterFightBubble { _0, _1, _2, _3, _4 })
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct MonsterFightBubble_4 {}

impl Decode for MonsterFightBubble_4 {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        Ok(MonsterFightBubble_4 {})
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct MonsterFightBubble_3 {}

impl Decode for MonsterFightBubble_3 {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        Ok(MonsterFightBubble_3 {})
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct MonsterFightBubble_2 {}

impl Decode for MonsterFightBubble_2 {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        Ok(MonsterFightBubble_2 {})
    }
}
