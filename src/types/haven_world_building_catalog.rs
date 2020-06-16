use crate::BinaryData;
use crate::decode::{Decode, DecodeState};
use std::io;
use std::marker::PhantomData;

#[derive(Debug, Clone, serde::Serialize)]
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

impl BinaryData for HavenWorldBuildingCatalog {
    fn id(_phantom: PhantomData<Self>) -> i32 {
        104
    }
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
        Ok(HavenWorldBuildingCatalog {
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

#[derive(Debug, Clone, serde::Serialize)]
pub struct HavenWorldBuildingCatalogBuildingCondition {
    pub building_type_needed: i32,
    pub quantity: i32,
}

impl Decode for HavenWorldBuildingCatalogBuildingCondition {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let building_type_needed = state.decode()?;
        let quantity = state.decode()?;
        Ok(HavenWorldBuildingCatalogBuildingCondition {
            building_type_needed,
            quantity,
        })
    }
}
