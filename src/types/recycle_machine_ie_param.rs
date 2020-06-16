use crate::BinaryData;
use crate::decode::{Decode, DecodeState};
use std::io;
use std::marker::PhantomData;

#[derive(Debug, Clone, serde::Serialize)]
pub struct RecycleMachineIeParam {
    pub id: i32,
    pub visual_mru_id: i32,
    pub _2: RecycleMachineIeParam_2,
}

impl BinaryData for RecycleMachineIeParam {
    fn id(_phantom: PhantomData<Self>) -> i32 {
        85
    }
}

impl Decode for RecycleMachineIeParam {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        let visual_mru_id = state.decode()?;
        let _2 = state.decode()?;
        Ok(RecycleMachineIeParam { id, visual_mru_id, _2 })
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct RecycleMachineIeParam_2 {
    pub _0: i8,
    pub _1: i32,
}

impl Decode for RecycleMachineIeParam_2 {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let _0 = state.decode()?;
        let _1 = state.decode()?;
        Ok(RecycleMachineIeParam_2 { _0, _1 })
    }
}
