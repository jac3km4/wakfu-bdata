use crate::BinaryData;
use crate::decode::{Decode, DecodeState};
use std::io;
use std::marker::PhantomData;

#[derive(Debug, Clone, serde::Serialize)]
pub struct NationColors {
    pub id: i32,
    pub color: String,
}

impl BinaryData for NationColors {
    fn id(_phantom: PhantomData<Self>) -> i32 {
        51
    }
}

impl Decode for NationColors {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        let color = state.decode()?;
        Ok(NationColors { id, color })
    }
}
