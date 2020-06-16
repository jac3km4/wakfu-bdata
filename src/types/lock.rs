use crate::BinaryData;
use crate::decode::{Decode, DecodeState};
use std::io;
use std::marker::PhantomData;

#[derive(Debug, Clone, serde::Serialize)]
pub struct Lock {
    pub id: i32,
    pub locked_item_id: i32,
    pub lock_value: i32,
    pub period_duration: i64,
    pub unlock_date: i64,
    pub available_for_citizens_only: bool,
}

impl BinaryData for Lock {
    fn id(_phantom: PhantomData<Self>) -> i32 {
        120
    }
}

impl Decode for Lock {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        let locked_item_id = state.decode()?;
        let lock_value = state.decode()?;
        let period_duration = state.decode()?;
        let unlock_date = state.decode()?;
        let available_for_citizens_only = state.decode()?;
        Ok(Lock {
            id,
            locked_item_id,
            lock_value,
            period_duration,
            unlock_date,
            available_for_citizens_only,
        })
    }
}
