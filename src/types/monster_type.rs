use crate::BinaryData;
use crate::decode::{Decode, DecodeState};
use std::io;
use std::marker::PhantomData;

#[derive(Debug, Clone, serde::Serialize)]
pub struct MonsterType {
    pub id: i32,
    pub parent_id: i32,
    pub kind: i8,
}

impl BinaryData for MonsterType {
    fn id(_phantom: PhantomData<Self>) -> i32 {
        47
    }
}

impl Decode for MonsterType {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        let parent_id = state.decode()?;
        let kind = state.decode()?;
        Ok(MonsterType { id, parent_id, kind })
    }
}
