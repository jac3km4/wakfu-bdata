use crate::BinaryData;
use crate::decode::{Decode, DecodeState};
use std::io;
use std::marker::PhantomData;

#[derive(Debug, Clone, serde::Serialize)]
pub struct InteractiveElementModel {
    pub id: i32,
    pub kind: i16,
    pub gfx: i32,
    pub color: i32,
    pub height: i8,
    pub particle_id: i32,
    pub particle_offset_z: i32,
}

impl BinaryData for InteractiveElementModel {
    fn id(_phantom: PhantomData<Self>) -> i32 {
        34
    }
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
        Ok(InteractiveElementModel {
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
