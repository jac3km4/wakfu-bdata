use std::io;

use serde::Serialize;

use crate::data::BinaryData;
use crate::decode::{Decode, DecodeState};

#[derive(Debug, Clone, Serialize)]
pub struct SkillAction {
    pub id: i32,
    pub mru_gfx_id: i32,
    pub mru_key: String,
    pub associated_items: Vec<i32>,
    pub anim_linkage: String,
}

impl Decode for SkillAction {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        let mru_gfx_id = state.decode()?;
        let mru_key = state.decode()?;
        let associated_items = state.decode()?;
        let anim_linkage = state.decode()?;
        Ok(Self {
            id,
            mru_gfx_id,
            mru_key,
            associated_items,
            anim_linkage,
        })
    }
}

impl BinaryData for SkillAction {
    const TYPE_ID: i16 = 65;
}
