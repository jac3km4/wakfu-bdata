use crate::BinaryData;
use crate::decode::{Decode, DecodeState};
use std::io;
use std::marker::PhantomData;

#[derive(Debug, Clone, serde::Serialize)]
pub struct Ground {
    pub id: i32,
    pub resource_fertility: std::collections::HashMap<i32, i16>,
    pub resource_type_fertility: std::collections::HashMap<i16, i16>,
}

impl BinaryData for Ground {
    fn id(_phantom: PhantomData<Self>) -> i32 {
        32
    }
}

impl Decode for Ground {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        let resource_fertility = state.decode()?;
        let resource_type_fertility = state.decode()?;
        Ok(Ground {
            id,
            resource_fertility,
            resource_type_fertility,
        })
    }
}
