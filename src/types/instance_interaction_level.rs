use std::io;

use serde::Serialize;

use crate::data::BinaryData;
use crate::decode::{Decode, DecodeState};

#[derive(Debug, Clone, Serialize)]
pub struct InstanceInteractionLevel {
    pub id: i32,
    pub world_id: i32,
    pub subscription_level: i32,
    pub interaction_level: i32,
}

impl Decode for InstanceInteractionLevel {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        let world_id = state.decode()?;
        let subscription_level = state.decode()?;
        let interaction_level = state.decode()?;
        Ok(Self {
            id,
            world_id,
            subscription_level,
            interaction_level,
        })
    }
}

impl BinaryData for InstanceInteractionLevel {
    const TYPE_ID: i16 = 134;
}
