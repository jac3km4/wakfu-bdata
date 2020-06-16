use crate::BinaryData;
use crate::decode::{Decode, DecodeState};
use std::io;
use std::marker::PhantomData;

#[derive(Debug, Clone, serde::Serialize)]
pub struct RegistrationEvent {
    pub _0: i32,
    pub _1: i32,
    pub _2: i32,
    pub _3: Vec<i64>,
    pub _4: i32,
    pub _5: i32,
}

impl BinaryData for RegistrationEvent {
    fn id(_phantom: PhantomData<Self>) -> i32 {
        59
    }
}

impl Decode for RegistrationEvent {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let _0 = state.decode()?;
        let _1 = state.decode()?;
        let _2 = state.decode()?;
        let _3 = state.decode()?;
        let _4 = state.decode()?;
        let _5 = state.decode()?;
        Ok(RegistrationEvent { _0, _1, _2, _3, _4, _5 })
    }
}
