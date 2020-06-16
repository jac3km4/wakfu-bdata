use crate::BinaryData;
use crate::decode::{Decode, DecodeState};
use std::io;
use std::marker::PhantomData;

#[derive(Debug, Clone, serde::Serialize)]
pub struct Bags {
    pub id: i32,
    pub capacity: i16,
    pub bag_type: i32,
    pub valid_item_categories: Vec<i32>,
}

impl BinaryData for Bags {
    fn id(_phantom: PhantomData<Self>) -> i32 {
        121
    }
}

impl Decode for Bags {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        let capacity = state.decode()?;
        let bag_type = state.decode()?;
        let valid_item_categories = state.decode()?;
        Ok(Bags {
            id,
            capacity,
            bag_type,
            valid_item_categories,
        })
    }
}
