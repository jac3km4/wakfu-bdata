use crate::BinaryData;
use crate::decode::{Decode, DecodeState};
use std::io;
use std::marker::PhantomData;

#[derive(Debug, Clone, serde::Serialize)]
pub struct CharacGainPerLevel {
    pub breed_id: i16,
    pub gains: std::collections::HashMap<i8, f32>,
}

impl BinaryData for CharacGainPerLevel {
    fn id(_phantom: PhantomData<Self>) -> i32 {
        16
    }
}

impl Decode for CharacGainPerLevel {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let breed_id = state.decode()?;
        let gains = state.decode()?;
        Ok(CharacGainPerLevel { breed_id, gains })
    }
}
