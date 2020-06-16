use crate::BinaryData;
use crate::decode::{Decode, DecodeState};
use std::io;
use std::marker::PhantomData;

#[derive(Debug, Clone, serde::Serialize)]
pub struct Chaos {
    pub _0: i32,
    pub _1: i16,
    pub _2: i16,
    pub _3: bool,
    pub _4: i8,
    pub _5: i8,
    pub _6: i8,
}

impl BinaryData for Chaos {
    fn id(_phantom: PhantomData<Self>) -> i32 {
        14
    }
}

impl Decode for Chaos {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let _0 = state.decode()?;
        let _1 = state.decode()?;
        let _2 = state.decode()?;
        let _3 = state.decode()?;
        let _4 = state.decode()?;
        let _5 = state.decode()?;
        let _6 = state.decode()?;
        Ok(Chaos {
            _0,
            _1,
            _2,
            _3,
            _4,
            _5,
            _6,
        })
    }
}
