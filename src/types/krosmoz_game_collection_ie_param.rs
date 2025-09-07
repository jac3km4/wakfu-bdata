use std::io;

use serde::Serialize;

use crate::data::BinaryData;
use crate::decode::{Decode, DecodeState};

#[derive(Debug, Clone, Serialize)]
pub struct KrosmozGameCollectionIeParam {
    pub id: i32,
    pub game_id: i8,
}

impl Decode for KrosmozGameCollectionIeParam {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        let game_id = state.decode()?;
        Ok(Self { id, game_id })
    }
}

impl BinaryData for KrosmozGameCollectionIeParam {
    const TYPE_ID: i16 = 109;
}
