use crate::BinaryData;
use crate::decode::{Decode, DecodeState};
use std::io;
use std::marker::PhantomData;

#[derive(Debug, Clone, serde::Serialize)]
pub struct BoatLink {
    pub id: i32,
    pub start: i32,
    pub end: i32,
    pub criteria: String,
    pub criteria_display: String,
    pub needs_to_pay_everytime: bool,
    pub _6: BoatLink_6,
}

impl BinaryData for BoatLink {
    fn id(_phantom: PhantomData<Self>) -> i32 {
        9
    }
}

impl Decode for BoatLink {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        let start = state.decode()?;
        let end = state.decode()?;
        let criteria = state.decode()?;
        let criteria_display = state.decode()?;
        let needs_to_pay_everytime = state.decode()?;
        let _6 = state.decode()?;
        Ok(BoatLink {
            id,
            start,
            end,
            criteria,
            criteria_display,
            needs_to_pay_everytime,
            _6,
        })
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct BoatLink_6 {
    pub _0: String,
    pub _1: i32,
    pub _2: i32,
    pub _3: i32,
}

impl Decode for BoatLink_6 {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let _0 = state.decode()?;
        let _1 = state.decode()?;
        let _2 = state.decode()?;
        let _3 = state.decode()?;
        Ok(BoatLink_6 { _0, _1, _2, _3 })
    }
}
