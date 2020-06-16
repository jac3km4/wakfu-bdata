use crate::BinaryData;
use crate::decode::{Decode, DecodeState};
use std::io;
use std::marker::PhantomData;

#[derive(Debug, Clone, serde::Serialize)]
pub struct AlmanachDate {
    pub id: i32,
    pub date: i64,
    pub almanach_entry_id: i32,
}

impl BinaryData for AlmanachDate {
    fn id(_phantom: PhantomData<Self>) -> i32 {
        99
    }
}

impl Decode for AlmanachDate {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        let date = state.decode()?;
        let almanach_entry_id = state.decode()?;
        Ok(AlmanachDate {
            id,
            date,
            almanach_entry_id,
        })
    }
}
