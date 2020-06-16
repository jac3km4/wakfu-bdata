use crate::BinaryData;
use crate::decode::{Decode, DecodeState};
use std::io;
use std::marker::PhantomData;

#[derive(Debug, Clone, serde::Serialize)]
pub struct ItemType {
    pub id: i16,
    pub parent_id: i16,
    pub visible_in_animations: bool,
    pub visible_in_market_place: bool,
    pub equipment_position: Vec<String>,
    pub disabled_equipement_position: Vec<String>,
    pub _6: bool,
    pub material_type: i16,
}

impl BinaryData for ItemType {
    fn id(_phantom: PhantomData<Self>) -> i32 {
        37
    }
}

impl Decode for ItemType {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        let parent_id = state.decode()?;
        let visible_in_animations = state.decode()?;
        let visible_in_market_place = state.decode()?;
        let equipment_position = state.decode()?;
        let disabled_equipement_position = state.decode()?;
        let _6 = state.decode()?;
        let material_type = state.decode()?;
        Ok(ItemType {
            id,
            parent_id,
            visible_in_animations,
            visible_in_market_place,
            equipment_position,
            disabled_equipement_position,
            _6,
            material_type,
        })
    }
}
