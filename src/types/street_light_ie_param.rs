use crate::BinaryData;
use crate::decode::{Decode, DecodeState};
use std::io;
use std::marker::PhantomData;

#[derive(Debug, Clone, serde::Serialize)]
pub struct StreetLightIeParam {
    pub id: i32,
    pub color: i32,
    pub range: f32,
    pub aps_id: i32,
    pub active_only_in_night: bool,
    pub ignition_visual_id: i32,
    pub ignition_use_object: bool,
    pub ignition_duration: i32,
    pub extinction_visual_id: i32,
    pub extinction_use_object: bool,
    pub extinction_duration: i32,
    pub _11: StreetLightIeParam_11,
}

impl BinaryData for StreetLightIeParam {
    fn id(_phantom: PhantomData<Self>) -> i32 {
        71
    }
}

impl Decode for StreetLightIeParam {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        let color = state.decode()?;
        let range = state.decode()?;
        let aps_id = state.decode()?;
        let active_only_in_night = state.decode()?;
        let ignition_visual_id = state.decode()?;
        let ignition_use_object = state.decode()?;
        let ignition_duration = state.decode()?;
        let extinction_visual_id = state.decode()?;
        let extinction_use_object = state.decode()?;
        let extinction_duration = state.decode()?;
        let _11 = state.decode()?;
        Ok(StreetLightIeParam {
            id,
            color,
            range,
            aps_id,
            active_only_in_night,
            ignition_visual_id,
            ignition_use_object,
            ignition_duration,
            extinction_visual_id,
            extinction_use_object,
            extinction_duration,
            _11,
        })
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct StreetLightIeParam_11 {
    pub _0: i8,
    pub _1: i32,
}

impl Decode for StreetLightIeParam_11 {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let _0 = state.decode()?;
        let _1 = state.decode()?;
        Ok(StreetLightIeParam_11 { _0, _1 })
    }
}
