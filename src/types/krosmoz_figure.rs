use std::io;

use serde::Serialize;

use crate::data::BinaryData;
use crate::decode::{Decode, DecodeState};

#[derive(Debug, Clone, Serialize)]
pub struct KrosmozFigure {
    pub id: i32,
    pub year: i32,
    pub addon: i32,
    pub season: i32,
}

impl Decode for KrosmozFigure {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        let year = state.decode()?;
        let addon = state.decode()?;
        let season = state.decode()?;
        Ok(Self {
            id,
            year,
            addon,
            season,
        })
    }
}

impl BinaryData for KrosmozFigure {
    const TYPE_ID: i16 = 113;
}
