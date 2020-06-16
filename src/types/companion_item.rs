use crate::BinaryData;
use crate::decode::{Decode, DecodeState};
use std::io;
use std::marker::PhantomData;

#[derive(Debug, Clone, serde::Serialize)]
pub struct CompanionItem {
    pub id: i32,
}

impl BinaryData for CompanionItem {
    fn id(_phantom: PhantomData<Self>) -> i32 {
        131
    }
}

impl Decode for CompanionItem {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        Ok(CompanionItem { id })
    }
}
