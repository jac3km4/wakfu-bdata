use crate::BinaryData;
use crate::decode::{Decode, DecodeState};
use std::io;
use std::marker::PhantomData;

#[derive(Debug, Clone, serde::Serialize)]
pub struct Teleporter {
    pub teleporter_id: i32,
    pub _1: Vec<Teleporter_1>,
}

impl BinaryData for Teleporter {
    fn id(_phantom: PhantomData<Self>) -> i32 {
        72
    }
}

impl Decode for Teleporter {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let teleporter_id = state.decode()?;
        let _1 = state.decode()?;
        Ok(Teleporter { teleporter_id, _1 })
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct Teleporter_1 {
    pub _0: i32,
    pub _1: i32,
    pub _2: i32,
    pub _3: i32,
    pub _4: i32,
    pub _5: i8,
    pub _6: String,
    pub _7: i32,
    pub _8: i32,
    pub _9: i16,
    pub _10: i32,
    pub _11: i16,
    pub _12: i16,
    pub _13: bool,
    pub _14: bool,
    pub _15: i32,
    pub _16: String,
    pub _17: i32,
    pub _18: i32,
    pub _19: i32,
}

impl Decode for Teleporter_1 {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let _0 = state.decode()?;
        let _1 = state.decode()?;
        let _2 = state.decode()?;
        let _3 = state.decode()?;
        let _4 = state.decode()?;
        let _5 = state.decode()?;
        let _6 = state.decode()?;
        let _7 = state.decode()?;
        let _8 = state.decode()?;
        let _9 = state.decode()?;
        let _10 = state.decode()?;
        let _11 = state.decode()?;
        let _12 = state.decode()?;
        let _13 = state.decode()?;
        let _14 = state.decode()?;
        let _15 = state.decode()?;
        let _16 = state.decode()?;
        let _17 = state.decode()?;
        let _18 = state.decode()?;
        let _19 = state.decode()?;
        Ok(Teleporter_1 {
            _0,
            _1,
            _2,
            _3,
            _4,
            _5,
            _6,
            _7,
            _8,
            _9,
            _10,
            _11,
            _12,
            _13,
            _14,
            _15,
            _16,
            _17,
            _18,
            _19,
        })
    }
}
