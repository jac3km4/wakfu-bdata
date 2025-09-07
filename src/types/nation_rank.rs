use std::io;

use serde::Serialize;

use crate::data::BinaryData;
use crate::decode::{Decode, DecodeState};

#[derive(Debug, Clone, Serialize)]
pub struct NationRank {
    pub id: i32,
    pub citizen_point_loss_factor: f32,
    pub criteria: String,
    pub citizen_score_line: i32,
}

impl Decode for NationRank {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        let citizen_point_loss_factor = state.decode()?;
        let criteria = state.decode()?;
        let citizen_score_line = state.decode()?;
        Ok(Self {
            id,
            citizen_point_loss_factor,
            criteria,
            citizen_score_line,
        })
    }
}

impl BinaryData for NationRank {
    const TYPE_ID: i16 = 53;
}
