use std::io;

use serde::Serialize;

use crate::data::BinaryData;
use crate::decode::{Decode, DecodeState};

#[derive(Debug, Clone, Serialize)]
pub struct ClimateBonus {
    pub buff_id: i32,
    pub gfx_id: i32,
    pub criteria: String,
    pub duration: i32,
    pub price: i16,
    pub temperature_difference: f32,
    pub rain_difference: f32,
}

impl Decode for ClimateBonus {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let buff_id = state.decode()?;
        let gfx_id = state.decode()?;
        let criteria = state.decode()?;
        let duration = state.decode()?;
        let price = state.decode()?;
        let temperature_difference = state.decode()?;
        let rain_difference = state.decode()?;
        Ok(Self {
            buff_id,
            gfx_id,
            criteria,
            duration,
            price,
            temperature_difference,
            rain_difference,
        })
    }
}

impl BinaryData for ClimateBonus {
    const TYPE_ID: i16 = 20;
}
