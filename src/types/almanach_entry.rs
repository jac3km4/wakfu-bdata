use crate::BinaryData;
use crate::decode::{Decode, DecodeState};
use std::io;
use std::marker::PhantomData;

#[derive(Debug, Clone, serde::Serialize)]
pub struct AlmanachEntry {
    pub id: i32,
    pub scenario_id: i32,
    pub achievement_id: i32,
    pub territories: Vec<i32>,
}

impl BinaryData for AlmanachEntry {
    fn id(_phantom: PhantomData<Self>) -> i32 {
        98
    }
}

impl Decode for AlmanachEntry {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        let scenario_id = state.decode()?;
        let achievement_id = state.decode()?;
        let territories = state.decode()?;
        Ok(AlmanachEntry {
            id,
            scenario_id,
            achievement_id,
            territories,
        })
    }
}
