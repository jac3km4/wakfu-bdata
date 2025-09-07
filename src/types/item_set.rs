use std::io;

use serde::Serialize;

use crate::data::BinaryData;
use crate::decode::{Decode, DecodeState};

#[derive(Debug, Clone, Serialize)]
pub struct ItemSet {
    pub id: i16,
    pub linked_item_reference_id: i32,
    pub items_id: Vec<i32>,
    pub effect_ids_by_part_count: std::collections::HashMap<i32, Vec<i32>>,
}

impl Decode for ItemSet {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        let linked_item_reference_id = state.decode()?;
        let items_id = state.decode()?;
        let effect_ids_by_part_count = state.decode()?;
        Ok(Self {
            id,
            linked_item_reference_id,
            items_id,
            effect_ids_by_part_count,
        })
    }
}

impl BinaryData for ItemSet {
    const TYPE_ID: i16 = 36;
}
