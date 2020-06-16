use crate::BinaryData;
use crate::decode::{Decode, DecodeState};
use std::io;
use std::marker::PhantomData;

#[derive(Debug, Clone, serde::Serialize)]
pub struct Drago {
    pub drago_id: i32,
    pub exit_x: i32,
    pub exit_y: i32,
    pub visual_id: i32,
    pub ui_gfx_id: i32,
    pub drago_criterion: String,
    pub landmark_travel_type: i8,
    pub _7: Drago_7,
}

impl BinaryData for Drago {
    fn id(_phantom: PhantomData<Self>) -> i32 {
        28
    }
}

impl Decode for Drago {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let drago_id = state.decode()?;
        let exit_x = state.decode()?;
        let exit_y = state.decode()?;
        let visual_id = state.decode()?;
        let ui_gfx_id = state.decode()?;
        let drago_criterion = state.decode()?;
        let landmark_travel_type = state.decode()?;
        let _7 = state.decode()?;
        Ok(Drago {
            drago_id,
            exit_x,
            exit_y,
            visual_id,
            ui_gfx_id,
            drago_criterion,
            landmark_travel_type,
            _7,
        })
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct Drago_7 {
    pub _0: String,
    pub _1: i32,
    pub _2: i32,
    pub _3: i32,
}

impl Decode for Drago_7 {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let _0 = state.decode()?;
        let _1 = state.decode()?;
        let _2 = state.decode()?;
        let _3 = state.decode()?;
        Ok(Drago_7 { _0, _1, _2, _3 })
    }
}
