use std::io;

use serde::Serialize;

use crate::data::BinaryData;
use crate::decode::{Decode, DecodeState};

#[derive(Debug, Clone, Serialize)]
pub struct RewardDisplayerIeParam {
    pub id: i32,
    pub item_ids: Vec<i32>,
}

impl Decode for RewardDisplayerIeParam {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        let item_ids = state.decode()?;
        Ok(Self { id, item_ids })
    }
}

impl BinaryData for RewardDisplayerIeParam {
    const TYPE_ID: i16 = 96;
}
