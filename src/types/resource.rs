use std::io;

use serde::Serialize;

use crate::data::BinaryData;
use crate::decode::{Decode, DecodeState};

#[derive(Debug, Clone, Serialize)]
pub struct Resource {
    pub id: i32,
    pub kind: i32,
    pub _2: i32,
    pub ideal_rain_min: i16,
    pub ideal_rain_max: i16,
    pub is_blocking: bool,
    pub use_big_challenge_aps: bool,
    pub is_monster_embryo: bool,
    pub _8: bool,
    pub monster_step_hatching: i16,
    pub properties: Vec<i32>,
    pub monster_families: Vec<i32>,
    pub _12: Vec<Resource_12>,
    pub height: i16,
    pub icon_gfx_id: i32,
    pub _15: std::collections::HashMap<i64, Vec<i32>>,
}

impl Decode for Resource {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        let kind = state.decode()?;
        let _2 = state.decode()?;
        let ideal_rain_min = state.decode()?;
        let ideal_rain_max = state.decode()?;
        let is_blocking = state.decode()?;
        let use_big_challenge_aps = state.decode()?;
        let is_monster_embryo = state.decode()?;
        let _8 = state.decode()?;
        let monster_step_hatching = state.decode()?;
        let properties = state.decode()?;
        let monster_families = state.decode()?;
        let _12 = state.decode()?;
        let height = state.decode()?;
        let icon_gfx_id = state.decode()?;
        let _15 = state.decode()?;
        Ok(Self {
            id,
            kind,
            _2,
            ideal_rain_min,
            ideal_rain_max,
            is_blocking,
            use_big_challenge_aps,
            is_monster_embryo,
            _8,
            monster_step_hatching,
            properties,
            monster_families,
            _12,
            height,
            icon_gfx_id,
            _15,
        })
    }
}

impl BinaryData for Resource {
    const TYPE_ID: i16 = 60;
}

#[derive(Debug, Clone, Serialize)]
pub struct Resource_12_1 {
    pub _0: i32,
    pub _1: i32,
    pub _2: i32,
    pub _3: i32,
    pub _4: i32,
    pub _5: i32,
    pub _6: i32,
    pub _7: i32,
    pub _8: i32,
    pub _9: String,
    pub _10: bool,
    pub _11: i32,
    pub _12: f32,
    pub _13: i32,
    pub _14: Vec<i32>,
    pub _15: i32,
    pub _16: bool,
}

impl Decode for Resource_12_1 {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let _0 = state.decode()?;
        let _1 = state.decode()?;
        let _2 = state.decode()?;
        let _3 = state.decode()?;
        let _4 = state.decode()?;
        let _5 = state.decode()?;
        let _6 = state.decode()?;
        let _7 = state.decode()?;
        let _8 = state.decode()?;
        let _9 = state.decode()?;
        let _10 = state.decode()?;
        let _11 = state.decode()?;
        let _12 = state.decode()?;
        let _13 = state.decode()?;
        let _14 = state.decode()?;
        let _15 = state.decode()?;
        let _16 = state.decode()?;
        Ok(Self {
            _0,
            _1,
            _2,
            _3,
            _4,
            _5,
            _6,
            _7,
            _8,
            _9,
            _10,
            _11,
            _12,
            _13,
            _14,
            _15,
            _16,
        })
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Resource_12 {
    pub _0: i32,
    pub _1: Vec<Resource_12_1>,
    pub _2: i32,
    pub _3: i32,
    pub _4: i32,
    pub _5: i32,
    pub _6: i32,
    pub _7: i32,
}

impl Decode for Resource_12 {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let _0 = state.decode()?;
        let _1 = state.decode()?;
        let _2 = state.decode()?;
        let _3 = state.decode()?;
        let _4 = state.decode()?;
        let _5 = state.decode()?;
        let _6 = state.decode()?;
        let _7 = state.decode()?;
        Ok(Self {
            _0,
            _1,
            _2,
            _3,
            _4,
            _5,
            _6,
            _7,
        })
    }
}
