use crate::BinaryData;
use crate::decode::{Decode, DecodeState};
use std::io;
use std::marker::PhantomData;

#[derive(Debug, Clone, serde::Serialize)]
pub struct Ability {
    pub bonus_id: i32,
    pub category_id: i32,
    pub max: i32,
    pub gfx_id: i32,
    pub effect_ids: Vec<i32>,
}

impl BinaryData for Ability {
    fn id(_phantom: PhantomData<Self>) -> i32 {
        135
    }
}

impl Decode for Ability {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let bonus_id = state.decode()?;
        let category_id = state.decode()?;
        let max = state.decode()?;
        let gfx_id = state.decode()?;
        let effect_ids = state.decode()?;
        Ok(Ability {
            bonus_id,
            category_id,
            max,
            gfx_id,
            effect_ids,
        })
    }
}
