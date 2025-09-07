use std::io;

use serde::Serialize;

use crate::data::BinaryData;
use crate::decode::{Decode, DecodeState};

#[derive(Debug, Clone, Serialize)]
pub struct InteractiveElementModel {
    pub id: i32,
    pub kind: i16,
    pub gfx: i32,
    pub color: i32,
    pub height: i8,
    pub particle_id: i32,
    pub particle_offset_z: i32,
}

impl Decode for InteractiveElementModel {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        let kind = state.decode()?;
        let gfx = state.decode()?;
        let color = state.decode()?;
        let height = state.decode()?;
        let particle_id = state.decode()?;
        let particle_offset_z = state.decode()?;
        Ok(Self {
            id,
            kind,
            gfx,
            color,
            height,
            particle_id,
            particle_offset_z,
        })
    }
}

impl BinaryData for InteractiveElementModel {
    const TYPE_ID: i16 = 34;
}
