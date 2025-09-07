use std::io;

use serde::Serialize;

use crate::data::BinaryData;
use crate::decode::{Decode, DecodeState};

#[derive(Debug, Clone, Serialize)]
pub struct Pet {
    pub id: i32,
    pub item_ref_id: i32,
    pub gfx_id: i32,
    pub item_color_ref_id: i32,
    pub item_reskin_ref_id: i32,
    pub min_meal_interval: i64,
    pub max_meal_interval: i64,
    pub xp_by_meal: i8,
    pub xp_per_level: i16,
    pub level_max: i16,
    pub health_penalties: Vec<PetHealthPenalties>,
    pub health_items: Vec<PetHealthItems>,
    pub meal_items: Vec<PetMealItems>,
    pub sleep_items: Vec<PetSleepItems>,
    pub equipment_items: Vec<i32>,
    pub color_items: Vec<PetColorItems>,
}

impl Decode for Pet {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        let item_ref_id = state.decode()?;
        let gfx_id = state.decode()?;
        let item_color_ref_id = state.decode()?;
        let item_reskin_ref_id = state.decode()?;
        let min_meal_interval = state.decode()?;
        let max_meal_interval = state.decode()?;
        let xp_by_meal = state.decode()?;
        let xp_per_level = state.decode()?;
        let level_max = state.decode()?;
        let health_penalties = state.decode()?;
        let health_items = state.decode()?;
        let meal_items = state.decode()?;
        let sleep_items = state.decode()?;
        let equipment_items = state.decode()?;
        let color_items = state.decode()?;
        Ok(Self {
            id,
            item_ref_id,
            gfx_id,
            item_color_ref_id,
            item_reskin_ref_id,
            min_meal_interval,
            max_meal_interval,
            xp_by_meal,
            xp_per_level,
            level_max,
            health_penalties,
            health_items,
            meal_items,
            sleep_items,
            equipment_items,
            color_items,
        })
    }
}

impl BinaryData for Pet {
    const TYPE_ID: i16 = 117;
}

#[derive(Debug, Clone, Serialize)]
pub struct PetHealthPenalties {
    pub penalty_type: i8,
    pub value: i8,
}

impl Decode for PetHealthPenalties {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let penalty_type = state.decode()?;
        let value = state.decode()?;
        Ok(Self {
            penalty_type,
            value,
        })
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct PetHealthItems {
    pub item_id: i32,
    pub value: i32,
}

impl Decode for PetHealthItems {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let item_id = state.decode()?;
        let value = state.decode()?;
        Ok(Self { item_id, value })
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct PetMealItems {
    pub item_id: i32,
    pub visible: bool,
}

impl Decode for PetMealItems {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let item_id = state.decode()?;
        let visible = state.decode()?;
        Ok(Self { item_id, visible })
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct PetSleepItems {
    pub item_id: i32,
    pub duration: i64,
}

impl Decode for PetSleepItems {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let item_id = state.decode()?;
        let duration = state.decode()?;
        Ok(Self { item_id, duration })
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct PetColorItems {
    pub item_id: i32,
    pub part_id: i32,
    pub color_abgr: i32,
}

impl Decode for PetColorItems {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let item_id = state.decode()?;
        let part_id = state.decode()?;
        let color_abgr = state.decode()?;
        Ok(Self {
            item_id,
            part_id,
            color_abgr,
        })
    }
}
