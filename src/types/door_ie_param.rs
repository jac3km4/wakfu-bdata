use crate::BinaryData;
use crate::decode::{Decode, DecodeState};
use std::io;
use std::marker::PhantomData;

#[derive(Debug, Clone, serde::Serialize)]
pub struct DoorIeParam {
    pub id: i32,
    pub visual_id: i32,
    pub consume_item: bool,
    pub item_needed: i32,
    pub kama_cost: i32,
    pub criterion: String,
}

impl BinaryData for DoorIeParam {
    fn id(_phantom: PhantomData<Self>) -> i32 {
        118
    }
}

impl Decode for DoorIeParam {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        let visual_id = state.decode()?;
        let consume_item = state.decode()?;
        let item_needed = state.decode()?;
        let kama_cost = state.decode()?;
        let criterion = state.decode()?;
        Ok(DoorIeParam {
            id,
            visual_id,
            consume_item,
            item_needed,
            kama_cost,
            criterion,
        })
    }
}
