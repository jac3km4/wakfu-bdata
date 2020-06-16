use crate::BinaryData;
use crate::decode::{Decode, DecodeState};
use std::io;
use std::marker::PhantomData;

#[derive(Debug, Clone, serde::Serialize)]
pub struct Cannon {
    pub cannon_id: i32,
    pub visual_id: i32,
    pub ui_gfx_id: i32,
    pub landmark_travel_type: i8,
    pub item_id: i32,
    pub item_qty: i32,
    pub _6: Vec<Cannon_6>,
}

impl BinaryData for Cannon {
    fn id(_phantom: PhantomData<Self>) -> i32 {
        12
    }
}

impl Decode for Cannon {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let cannon_id = state.decode()?;
        let visual_id = state.decode()?;
        let ui_gfx_id = state.decode()?;
        let landmark_travel_type = state.decode()?;
        let item_id = state.decode()?;
        let item_qty = state.decode()?;
        let _6 = state.decode()?;
        Ok(Cannon {
            cannon_id,
            visual_id,
            ui_gfx_id,
            landmark_travel_type,
            item_id,
            item_qty,
            _6,
        })
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct Cannon_6_6 {
    pub _0: String,
    pub _1: i32,
    pub _2: i32,
    pub _3: i32,
}

impl Decode for Cannon_6_6 {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let _0 = state.decode()?;
        let _1 = state.decode()?;
        let _2 = state.decode()?;
        let _3 = state.decode()?;
        Ok(Cannon_6_6 { _0, _1, _2, _3 })
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct Cannon_6 {
    pub _0: i32,
    pub _1: i32,
    pub _2: i32,
    pub _3: i32,
    pub _4: i32,
    pub _5: String,
    pub _6: Cannon_6_6,
}

impl Decode for Cannon_6 {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let _0 = state.decode()?;
        let _1 = state.decode()?;
        let _2 = state.decode()?;
        let _3 = state.decode()?;
        let _4 = state.decode()?;
        let _5 = state.decode()?;
        let _6 = state.decode()?;
        Ok(Cannon_6 {
            _0,
            _1,
            _2,
            _3,
            _4,
            _5,
            _6,
        })
    }
}
