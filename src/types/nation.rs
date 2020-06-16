use crate::BinaryData;
use crate::decode::{Decode, DecodeState};
use std::io;
use std::marker::PhantomData;

#[derive(Debug, Clone, serde::Serialize)]
pub struct Nation {
    pub _0: i32,
    pub _1: i16,
    pub _2: i16,
    pub _3: i16,
    pub _4: i16,
    pub _5: i32,
    pub _6: i32,
    pub _7: i16,
    pub _8: i32,
    pub _9: i32,
}

impl BinaryData for Nation {
    fn id(_phantom: PhantomData<Self>) -> i32 {
        50
    }
}

impl Decode for Nation {
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
        Ok(Nation {
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
        })
    }
}
