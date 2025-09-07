use std::io;

use serde::Serialize;

use crate::data::BinaryData;
use crate::decode::{Decode, DecodeState};

#[derive(Debug, Clone, Serialize)]
pub struct Lock {
    pub id: i32,
    pub locked_item_id: i32,
    pub lock_value: i32,
    pub period_duration: i64,
    pub unlock_date: i64,
    pub available_for_citizens_only: bool,
}

impl Decode for Lock {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        let locked_item_id = state.decode()?;
        let lock_value = state.decode()?;
        let period_duration = state.decode()?;
        let unlock_date = state.decode()?;
        let available_for_citizens_only = state.decode()?;
        Ok(Self {
            id,
            locked_item_id,
            lock_value,
            period_duration,
            unlock_date,
            available_for_citizens_only,
        })
    }
}

impl BinaryData for Lock {
    const TYPE_ID: i16 = 120;
}
