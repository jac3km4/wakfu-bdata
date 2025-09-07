use std::io;

use serde::Serialize;

use crate::data::BinaryData;
use crate::decode::{Decode, DecodeState};

#[derive(Debug, Clone, Serialize)]
pub struct LootChestIeParam {
    pub id: i32,
    pub visual_id: i32,
    pub cooldown: i32,
    pub cost: i32,
    pub item_id_cost: i32,
    pub item_quantity_cost: i32,
    pub do_consume_item: bool,
    pub nb_activation: i32,
    pub _8: bool,
    pub distribution_duration: i32,
    pub criteria: String,
    pub _11: LootChestIeParam_11,
}

impl Decode for LootChestIeParam {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        let visual_id = state.decode()?;
        let cooldown = state.decode()?;
        let cost = state.decode()?;
        let item_id_cost = state.decode()?;
        let item_quantity_cost = state.decode()?;
        let do_consume_item = state.decode()?;
        let nb_activation = state.decode()?;
        let _8 = state.decode()?;
        let distribution_duration = state.decode()?;
        let criteria = state.decode()?;
        let _11 = state.decode()?;
        Ok(Self {
            id,
            visual_id,
            cooldown,
            cost,
            item_id_cost,
            item_quantity_cost,
            do_consume_item,
            nb_activation,
            _8,
            distribution_duration,
            criteria,
            _11,
        })
    }
}

impl BinaryData for LootChestIeParam {
    const TYPE_ID: i16 = 38;
}

#[derive(Debug, Clone, Serialize)]
pub struct LootChestIeParam_11 {
    pub _0: i8,
    pub _1: i32,
}

impl Decode for LootChestIeParam_11 {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let _0 = state.decode()?;
        let _1 = state.decode()?;
        Ok(Self { _0, _1 })
    }
}
