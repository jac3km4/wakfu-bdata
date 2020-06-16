use crate::BinaryData;
use crate::decode::{Decode, DecodeState};
use std::io;
use std::marker::PhantomData;

#[derive(Debug, Clone, serde::Serialize)]
pub struct RewardDisplayerIeParam {
    pub id: i32,
    pub item_ids: Vec<i32>,
}

impl BinaryData for RewardDisplayerIeParam {
    fn id(_phantom: PhantomData<Self>) -> i32 {
        96
    }
}

impl Decode for RewardDisplayerIeParam {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        let item_ids = state.decode()?;
        Ok(RewardDisplayerIeParam { id, item_ids })
    }
}
