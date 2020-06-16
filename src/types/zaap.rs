use crate::BinaryData;
use crate::decode::{Decode, DecodeState};
use std::io;
use std::marker::PhantomData;

#[derive(Debug, Clone, serde::Serialize)]
pub struct Zaap {
    pub zaap_id: i32,
    pub exit_x: i32,
    pub exit_y: i32,
    pub exit_world_id: i32,
    pub visual_id: i32,
    pub ui_gfx_id: i32,
    pub landmark_travel_type: i8,
    pub zaap_base: bool,
    pub destination_criteria: String,
    pub _9: i32,
    pub _10: i32,
    pub _11: Zaap_11,
}

impl BinaryData for Zaap {
    fn id(_phantom: PhantomData<Self>) -> i32 {
        76
    }
}

impl Decode for Zaap {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let zaap_id = state.decode()?;
        let exit_x = state.decode()?;
        let exit_y = state.decode()?;
        let exit_world_id = state.decode()?;
        let visual_id = state.decode()?;
        let ui_gfx_id = state.decode()?;
        let landmark_travel_type = state.decode()?;
        let zaap_base = state.decode()?;
        let destination_criteria = state.decode()?;
        let _9 = state.decode()?;
        let _10 = state.decode()?;
        let _11 = state.decode()?;
        Ok(Zaap {
            zaap_id,
            exit_x,
            exit_y,
            exit_world_id,
            visual_id,
            ui_gfx_id,
            landmark_travel_type,
            zaap_base,
            destination_criteria,
            _9,
            _10,
            _11,
        })
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct Zaap_11 {
    pub _0: String,
    pub _1: i32,
    pub _2: i32,
    pub _3: i32,
}

impl Decode for Zaap_11 {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let _0 = state.decode()?;
        let _1 = state.decode()?;
        let _2 = state.decode()?;
        let _3 = state.decode()?;
        Ok(Zaap_11 { _0, _1, _2, _3 })
    }
}
