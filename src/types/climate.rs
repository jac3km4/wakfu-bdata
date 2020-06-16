use crate::BinaryData;
use crate::decode::{Decode, DecodeState};
use std::io;
use std::marker::PhantomData;

#[derive(Debug, Clone, serde::Serialize)]
pub struct Climate {
    pub _0: i32,
    pub _1: i8,
    pub _2: i8,
    pub _3: i8,
    pub _4: i8,
    pub _5: f32,
    pub _6: f32,
    pub _7: i32,
    pub _8: i32,
    pub _9: i32,
    pub _10: i32,
}

impl BinaryData for Climate {
    fn id(_phantom: PhantomData<Self>) -> i32 {
        19
    }
}

impl Decode for Climate {
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
        Ok(Climate {
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
        })
    }
}
