use crate::BinaryData;
use crate::decode::{Decode, DecodeState};
use std::io;
use std::marker::PhantomData;

#[derive(Debug, Clone, serde::Serialize)]
pub struct Craft {
    pub craft_id: i32,
    pub book_item_id: i32,
    pub xp_factor: f32,
    pub innate: bool,
    pub conceptual_craft: bool,
    pub hidden_craft: bool,
}

impl BinaryData for Craft {
    fn id(_phantom: PhantomData<Self>) -> i32 {
        23
    }
}

impl Decode for Craft {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let craft_id = state.decode()?;
        let book_item_id = state.decode()?;
        let xp_factor = state.decode()?;
        let innate = state.decode()?;
        let conceptual_craft = state.decode()?;
        let hidden_craft = state.decode()?;
        Ok(Craft {
            craft_id,
            book_item_id,
            xp_factor,
            innate,
            conceptual_craft,
            hidden_craft,
        })
    }
}
