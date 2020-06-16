use crate::BinaryData;
use crate::decode::{Decode, DecodeState};
use std::io;
use std::marker::PhantomData;

#[derive(Debug, Clone, serde::Serialize)]
pub struct CraftIeParam {
    pub id: i32,
    pub aps_id: i32,
    pub visual_mru_id: i32,
    pub skill_id: i32,
    pub allowed_recipes: Vec<i32>,
    pub _5: CraftIeParam_5,
}

impl BinaryData for CraftIeParam {
    fn id(_phantom: PhantomData<Self>) -> i32 {
        24
    }
}

impl Decode for CraftIeParam {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        let aps_id = state.decode()?;
        let visual_mru_id = state.decode()?;
        let skill_id = state.decode()?;
        let allowed_recipes = state.decode()?;
        let _5 = state.decode()?;
        Ok(CraftIeParam {
            id,
            aps_id,
            visual_mru_id,
            skill_id,
            allowed_recipes,
            _5,
        })
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct CraftIeParam_5 {
    pub _0: i8,
    pub _1: i32,
}

impl Decode for CraftIeParam_5 {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let _0 = state.decode()?;
        let _1 = state.decode()?;
        Ok(CraftIeParam_5 { _0, _1 })
    }
}
