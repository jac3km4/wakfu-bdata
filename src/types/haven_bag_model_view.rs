use std::io;

use serde::Serialize;

use crate::data::BinaryData;
use crate::decode::{Decode, DecodeState};

#[derive(Debug, Clone, Serialize)]
pub struct HavenBagModelView {
    pub id: i32,
    pub restriction_world: bool,
    pub restriction_market: bool,
    pub background_map_id: i32,
    pub innate: bool,
    pub _5: f32,
    pub _6: f32,
    pub _7: f32,
}

impl Decode for HavenBagModelView {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        let restriction_world = state.decode()?;
        let restriction_market = state.decode()?;
        let background_map_id = state.decode()?;
        let innate = state.decode()?;
        let _5 = state.decode()?;
        let _6 = state.decode()?;
        let _7 = state.decode()?;
        Ok(Self {
            id,
            restriction_world,
            restriction_market,
            background_map_id,
            innate,
            _5,
            _6,
            _7,
        })
    }
}

impl BinaryData for HavenBagModelView {
    const TYPE_ID: i16 = 33;
}
