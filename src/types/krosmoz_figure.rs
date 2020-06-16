use crate::BinaryData;
use crate::decode::{Decode, DecodeState};
use std::io;
use std::marker::PhantomData;

#[derive(Debug, Clone, serde::Serialize)]
pub struct KrosmozFigure {
    pub id: i32,
    pub year: i32,
    pub addon: i32,
    pub season: i32,
}

impl BinaryData for KrosmozFigure {
    fn id(_phantom: PhantomData<Self>) -> i32 {
        113
    }
}

impl Decode for KrosmozFigure {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        let year = state.decode()?;
        let addon = state.decode()?;
        let season = state.decode()?;
        Ok(KrosmozFigure {
            id,
            year,
            addon,
            season,
        })
    }
}
