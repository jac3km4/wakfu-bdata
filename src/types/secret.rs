use crate::BinaryData;
use crate::decode::{Decode, DecodeState};
use std::io;
use std::marker::PhantomData;

#[derive(Debug, Clone, serde::Serialize)]
pub struct Secret {
    pub id: i32,
    pub level: i16,
    pub item_id: i16,
}

impl BinaryData for Secret {
    fn id(_phantom: PhantomData<Self>) -> i32 {
        133
    }
}

impl Decode for Secret {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        let level = state.decode()?;
        let item_id = state.decode()?;
        Ok(Secret { id, level, item_id })
    }
}
