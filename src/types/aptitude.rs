use crate::BinaryData;
use crate::decode::{Decode, DecodeState};
use std::io;
use std::marker::PhantomData;

#[derive(Debug, Clone, serde::Serialize)]
pub struct Aptitude {
    pub id: i32,
    pub breed_id: i16,
    pub characteristic_id: i8,
    pub ui_id: i32,
    pub aptitude_gfx_id: i32,
    pub spell_gfx_id: i32,
    pub max_level: i16,
    pub constant_cost: i32,
    pub variable_cost: Vec<i32>,
    pub linked_spell_id: i32,
    pub level_unlock: Vec<i32>,
    pub effect_ids: Vec<i32>,
}

impl BinaryData for Aptitude {
    fn id(_phantom: PhantomData<Self>) -> i32 {
        78
    }
}

impl Decode for Aptitude {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        let breed_id = state.decode()?;
        let characteristic_id = state.decode()?;
        let ui_id = state.decode()?;
        let aptitude_gfx_id = state.decode()?;
        let spell_gfx_id = state.decode()?;
        let max_level = state.decode()?;
        let constant_cost = state.decode()?;
        let variable_cost = state.decode()?;
        let linked_spell_id = state.decode()?;
        let level_unlock = state.decode()?;
        let effect_ids = state.decode()?;
        Ok(Aptitude {
            id,
            breed_id,
            characteristic_id,
            ui_id,
            aptitude_gfx_id,
            spell_gfx_id,
            max_level,
            constant_cost,
            variable_cost,
            linked_spell_id,
            level_unlock,
            effect_ids,
        })
    }
}
