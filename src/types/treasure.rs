use crate::BinaryData;
use crate::decode::{Decode, DecodeState};
use std::io;
use std::marker::PhantomData;

#[derive(Debug, Clone, serde::Serialize)]
pub struct Treasure {
    pub id: i32,
    pub used_item: i32,
    pub reward_item: i32,
    pub quantity: i16,
    pub reward_kama: i32,
    pub reward_loot_chest: i32,
    pub duration: i32,
    pub criterion: String,
    pub win_percent: f32,
}

impl BinaryData for Treasure {
    fn id(_phantom: PhantomData<Self>) -> i32 {
        122
    }
}

impl Decode for Treasure {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        let used_item = state.decode()?;
        let reward_item = state.decode()?;
        let quantity = state.decode()?;
        let reward_kama = state.decode()?;
        let reward_loot_chest = state.decode()?;
        let duration = state.decode()?;
        let criterion = state.decode()?;
        let win_percent = state.decode()?;
        Ok(Treasure {
            id,
            used_item,
            reward_item,
            quantity,
            reward_kama,
            reward_loot_chest,
            duration,
            criterion,
            win_percent,
        })
    }
}
