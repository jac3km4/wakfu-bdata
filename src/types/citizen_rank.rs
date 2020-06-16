use crate::BinaryData;
use crate::decode::{Decode, DecodeState};
use std::io;
use std::marker::PhantomData;

#[derive(Debug, Clone, serde::Serialize)]
pub struct CitizenRank {
    pub id: i32,
    pub cap: i32,
    pub pdc_loss_factor: i32,
    pub translation_key: String,
    pub color: String,
    pub rules: Vec<i32>,
}

impl BinaryData for CitizenRank {
    fn id(_phantom: PhantomData<Self>) -> i32 {
        17
    }
}

impl Decode for CitizenRank {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        let cap = state.decode()?;
        let pdc_loss_factor = state.decode()?;
        let translation_key = state.decode()?;
        let color = state.decode()?;
        let rules = state.decode()?;
        Ok(CitizenRank {
            id,
            cap,
            pdc_loss_factor,
            translation_key,
            color,
            rules,
        })
    }
}
