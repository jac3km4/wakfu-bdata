use std::io;

use serde::Serialize;

use crate::data::BinaryData;
use crate::decode::{Decode, DecodeState};

#[derive(Debug, Clone, Serialize)]
pub struct Boat {
    pub boat_id: i32,
    pub exit_x: i32,
    pub exit_y: i32,
    pub exit_world_id: i32,
    pub visual_id: i32,
    pub ui_gfx_id: i32,
    pub landmark_travel_type: i8,
}

impl Decode for Boat {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let boat_id = state.decode()?;
        let exit_x = state.decode()?;
        let exit_y = state.decode()?;
        let exit_world_id = state.decode()?;
        let visual_id = state.decode()?;
        let ui_gfx_id = state.decode()?;
        let landmark_travel_type = state.decode()?;
        Ok(Self {
            boat_id,
            exit_x,
            exit_y,
            exit_world_id,
            visual_id,
            ui_gfx_id,
            landmark_travel_type,
        })
    }
}

impl BinaryData for Boat {
    const TYPE_ID: i16 = 8;
}
