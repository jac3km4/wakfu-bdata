use std::io;

use serde::Serialize;

use crate::data::BinaryData;
use crate::decode::{Decode, DecodeState};

#[derive(Debug, Clone, Serialize)]
pub struct FixedAbilityCategory {
    pub category_id: i32,
    pub levels: Vec<i32>,
    pub bonus_ids: Vec<i32>,
}

impl Decode for FixedAbilityCategory {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let category_id = state.decode()?;
        let levels = state.decode()?;
        let bonus_ids = state.decode()?;
        Ok(Self {
            category_id,
            levels,
            bonus_ids,
        })
    }
}

impl BinaryData for FixedAbilityCategory {
    const TYPE_ID: i16 = 136;
}
