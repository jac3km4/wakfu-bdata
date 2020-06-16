use crate::BinaryData;
use crate::decode::{Decode, DecodeState};
use std::io;
use std::marker::PhantomData;

#[derive(Debug, Clone, serde::Serialize)]
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

impl BinaryData for HavenBagModelView {
    fn id(_phantom: PhantomData<Self>) -> i32 {
        33
    }
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
        Ok(HavenBagModelView {
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
