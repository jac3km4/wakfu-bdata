use std::io;

use serde::Serialize;

use crate::data::BinaryData;
use crate::decode::{Decode, DecodeState};

#[derive(Debug, Clone, Serialize)]
pub struct TimelineBuffList {
    pub id: i32,
    pub kind_id: i32,
    pub gfx_id: i32,
    pub for_player: bool,
    pub effect_ids: Vec<i32>,
}

impl Decode for TimelineBuffList {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        let kind_id = state.decode()?;
        let gfx_id = state.decode()?;
        let for_player = state.decode()?;
        let effect_ids = state.decode()?;
        Ok(Self {
            id,
            kind_id,
            gfx_id,
            for_player,
            effect_ids,
        })
    }
}

impl BinaryData for TimelineBuffList {
    const TYPE_ID: i16 = 73;
}
