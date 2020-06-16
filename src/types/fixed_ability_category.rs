use crate::BinaryData;
use crate::decode::{Decode, DecodeState};
use std::io;
use std::marker::PhantomData;

#[derive(Debug, Clone, serde::Serialize)]
pub struct FixedAbilityCategory {
    pub category_id: i32,
    pub levels: Vec<i32>,
    pub bonus_ids: Vec<i32>,
}

impl BinaryData for FixedAbilityCategory {
    fn id(_phantom: PhantomData<Self>) -> i32 {
        136
    }
}

impl Decode for FixedAbilityCategory {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let category_id = state.decode()?;
        let levels = state.decode()?;
        let bonus_ids = state.decode()?;
        Ok(FixedAbilityCategory {
            category_id,
            levels,
            bonus_ids,
        })
    }
}
