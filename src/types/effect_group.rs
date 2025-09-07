use std::io;

use serde::Serialize;

use crate::data::BinaryData;
use crate::decode::{Decode, DecodeState};

#[derive(Debug, Clone, Serialize)]
pub struct EffectGroup {
    pub id: i32,
    pub effect_ids: Vec<i32>,
}

impl Decode for EffectGroup {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        let effect_ids = state.decode()?;
        Ok(Self { id, effect_ids })
    }
}

impl BinaryData for EffectGroup {
    const TYPE_ID: i16 = 29;
}
