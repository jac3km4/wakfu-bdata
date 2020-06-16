use crate::BinaryData;
use crate::decode::{Decode, DecodeState};
use std::io;
use std::marker::PhantomData;

#[derive(Debug, Clone, serde::Serialize)]
pub struct ClimateBonus {
    pub buff_id: i32,
    pub gfx_id: i32,
    pub criteria: String,
    pub duration: i32,
    pub price: i16,
    pub temperature_difference: f32,
    pub rain_difference: f32,
}

impl BinaryData for ClimateBonus {
    fn id(_phantom: PhantomData<Self>) -> i32 {
        20
    }
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
        Ok(ClimateBonus {
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
