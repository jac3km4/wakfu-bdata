use std::io;

use serde::Serialize;

use crate::data::BinaryData;
use crate::decode::{Decode, DecodeState};

#[derive(Debug, Clone, Serialize)]
pub struct HavenWorldBuildingCatalog {
    pub id: i32,
    pub order: i32,
    pub building_type: i32,
    pub category_id: i32,
    pub buyable: bool,
    pub max_quantity: i16,
    pub is_deco_only: bool,
    pub building_sound_id: i32,
    pub building_condition: Vec<HavenWorldBuildingCatalogBuildingCondition>,
}

impl Decode for HavenWorldBuildingCatalog {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        let order = state.decode()?;
        let building_type = state.decode()?;
        let category_id = state.decode()?;
        let buyable = state.decode()?;
        let max_quantity = state.decode()?;
        let is_deco_only = state.decode()?;
        let building_sound_id = state.decode()?;
        let building_condition = state.decode()?;
        Ok(Self {
            id,
            order,
            building_type,
            category_id,
            buyable,
            max_quantity,
            is_deco_only,
            building_sound_id,
            building_condition,
        })
    }
}

impl BinaryData for HavenWorldBuildingCatalog {
    const TYPE_ID: i16 = 104;
}

#[derive(Debug, Clone, Serialize)]
pub struct HavenWorldBuildingCatalogBuildingCondition {
    pub building_type_needed: i32,
    pub quantity: i32,
}

impl Decode for HavenWorldBuildingCatalogBuildingCondition {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let building_type_needed = state.decode()?;
        let quantity = state.decode()?;
        Ok(Self {
            building_type_needed,
            quantity,
        })
    }
}
