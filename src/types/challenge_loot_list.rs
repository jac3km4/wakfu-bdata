use crate::BinaryData;
use crate::decode::{Decode, DecodeState};
use std::io;
use std::marker::PhantomData;

#[derive(Debug, Clone, serde::Serialize)]
pub struct ChallengeLootList {
    pub id: i32,
    pub entries: Vec<ChallengeLootListEntries>,
}

impl BinaryData for ChallengeLootList {
    fn id(_phantom: PhantomData<Self>) -> i32 {
        80
    }
}

impl Decode for ChallengeLootList {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        let entries = state.decode()?;
        Ok(ChallengeLootList { id, entries })
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct ChallengeLootListEntries {
    pub challenge_id: i32,
    pub criteria: String,
}

impl Decode for ChallengeLootListEntries {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let challenge_id = state.decode()?;
        let criteria = state.decode()?;
        Ok(ChallengeLootListEntries { challenge_id, criteria })
    }
}
