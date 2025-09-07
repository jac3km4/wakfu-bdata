use std::io;

use serde::Serialize;

use crate::data::BinaryData;
use crate::decode::{Decode, DecodeState};

#[derive(Debug, Clone, Serialize)]
pub struct CitizenRank {
    pub id: i32,
    pub cap: i32,
    pub pdc_loss_factor: i32,
    pub translation_key: String,
    pub color: String,
    pub rules: Vec<i32>,
}

impl Decode for CitizenRank {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        let cap = state.decode()?;
        let pdc_loss_factor = state.decode()?;
        let translation_key = state.decode()?;
        let color = state.decode()?;
        let rules = state.decode()?;
        Ok(Self {
            id,
            cap,
            pdc_loss_factor,
            translation_key,
            color,
            rules,
        })
    }
}

impl BinaryData for CitizenRank {
    const TYPE_ID: i16 = 17;
}
