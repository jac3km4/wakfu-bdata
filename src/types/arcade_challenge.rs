use crate::BinaryData;
use crate::decode::{Decode, DecodeState};
use std::io;
use std::marker::PhantomData;

#[derive(Debug, Clone, serde::Serialize)]
pub struct ArcadeChallenge {
    pub id: i32,
}

impl BinaryData for ArcadeChallenge {
    fn id(_phantom: PhantomData<Self>) -> i32 {
        90
    }
}

impl Decode for ArcadeChallenge {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        Ok(ArcadeChallenge { id })
    }
}
