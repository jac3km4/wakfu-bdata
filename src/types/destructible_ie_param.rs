use std::io;

use serde::Serialize;

use crate::data::BinaryData;
use crate::decode::{Decode, DecodeState};

#[derive(Debug, Clone, Serialize)]
pub struct DestructibleIeParam {
    pub id: i32,
    pub pdv: i32,
    pub regen_delay: i32,
    pub res_water: i32,
    pub res_fire: i32,
    pub res_earth: i32,
    pub res_wind: i32,
    pub effect_ids: Vec<i32>,
}

impl Decode for DestructibleIeParam {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        let pdv = state.decode()?;
        let regen_delay = state.decode()?;
        let res_water = state.decode()?;
        let res_fire = state.decode()?;
        let res_earth = state.decode()?;
        let res_wind = state.decode()?;
        let effect_ids = state.decode()?;
        Ok(Self {
            id,
            pdv,
            regen_delay,
            res_water,
            res_fire,
            res_earth,
            res_wind,
            effect_ids,
        })
    }
}

impl BinaryData for DestructibleIeParam {
    const TYPE_ID: i16 = 26;
}
