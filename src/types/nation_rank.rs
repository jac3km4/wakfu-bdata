use crate::BinaryData;
use crate::decode::{Decode, DecodeState};
use std::io;
use std::marker::PhantomData;

#[derive(Debug, Clone, serde::Serialize)]
pub struct NationRank {
    pub id: i32,
    pub citizen_point_loss_factor: f32,
    pub criteria: String,
    pub citizen_score_line: i32,
}

impl BinaryData for NationRank {
    fn id(_phantom: PhantomData<Self>) -> i32 {
        53
    }
}

impl Decode for NationRank {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        let citizen_point_loss_factor = state.decode()?;
        let criteria = state.decode()?;
        let citizen_score_line = state.decode()?;
        Ok(NationRank {
            id,
            citizen_point_loss_factor,
            criteria,
            citizen_score_line,
        })
    }
}
