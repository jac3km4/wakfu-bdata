use crate::BinaryData;
use crate::decode::{Decode, DecodeState};
use std::io;
use std::marker::PhantomData;

#[derive(Debug, Clone, serde::Serialize)]
pub struct ClimateBonusList {
    pub buff_list_id: i32,
    pub entries: Vec<i32>,
}

impl BinaryData for ClimateBonusList {
    fn id(_phantom: PhantomData<Self>) -> i32 {
        21
    }
}

impl Decode for ClimateBonusList {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let buff_list_id = state.decode()?;
        let entries = state.decode()?;
        Ok(ClimateBonusList { buff_list_id, entries })
    }
}
