use crate::BinaryData;
use crate::decode::{Decode, DecodeState};
use std::io;
use std::marker::PhantomData;

#[derive(Debug, Clone, serde::Serialize)]
pub struct InstanceInteractionLevel {
    pub id: i32,
    pub world_id: i32,
    pub subscription_level: i32,
    pub interaction_level: i32,
}

impl BinaryData for InstanceInteractionLevel {
    fn id(_phantom: PhantomData<Self>) -> i32 {
        134
    }
}

impl Decode for InstanceInteractionLevel {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        let world_id = state.decode()?;
        let subscription_level = state.decode()?;
        let interaction_level = state.decode()?;
        Ok(InstanceInteractionLevel {
            id,
            world_id,
            subscription_level,
            interaction_level,
        })
    }
}
