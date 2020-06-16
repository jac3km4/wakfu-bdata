use crate::BinaryData;
use crate::decode::{Decode, DecodeState};
use std::io;
use std::marker::PhantomData;

#[derive(Debug, Clone, serde::Serialize)]
pub struct Skill {
    pub id: i32,
    pub kind: i32,
    pub script_id: i32,
    pub mru_gfx_id: i32,
    pub mru_key: String,
    pub anim_linkage: String,
    pub associated_item_types: Vec<i32>,
    pub associated_items: Vec<i32>,
    pub max_level: i32,
    pub is_innate: bool,
}

impl BinaryData for Skill {
    fn id(_phantom: PhantomData<Self>) -> i32 {
        64
    }
}

impl Decode for Skill {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        let kind = state.decode()?;
        let script_id = state.decode()?;
        let mru_gfx_id = state.decode()?;
        let mru_key = state.decode()?;
        let anim_linkage = state.decode()?;
        let associated_item_types = state.decode()?;
        let associated_items = state.decode()?;
        let max_level = state.decode()?;
        let is_innate = state.decode()?;
        Ok(Skill {
            id,
            kind,
            script_id,
            mru_gfx_id,
            mru_key,
            anim_linkage,
            associated_item_types,
            associated_items,
            max_level,
            is_innate,
        })
    }
}
