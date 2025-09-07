use std::io;

use serde::Serialize;

use crate::data::BinaryData;
use crate::decode::{Decode, DecodeState};

#[derive(Debug, Clone, Serialize)]
pub struct Bags {
    pub id: i32,
    pub capacity: i16,
    pub bag_type: i32,
    pub valid_item_categories: Vec<i32>,
}

impl Decode for Bags {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        let capacity = state.decode()?;
        let bag_type = state.decode()?;
        let valid_item_categories = state.decode()?;
        Ok(Self {
            id,
            capacity,
            bag_type,
            valid_item_categories,
        })
    }
}

impl BinaryData for Bags {
    const TYPE_ID: i16 = 121;
}
