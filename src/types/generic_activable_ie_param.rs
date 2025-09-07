use std::io;

use serde::Serialize;

use crate::data::BinaryData;
use crate::decode::{Decode, DecodeState};

#[derive(Debug, Clone, Serialize)]
pub struct GenericActivableIeParam {
    pub id: i32,
    pub visuals: Vec<GenericActivableIeParamVisuals>,
    pub _2: GenericActivableIeParam_2,
}

impl Decode for GenericActivableIeParam {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        let visuals = state.decode()?;
        let _2 = state.decode()?;
        Ok(Self { id, visuals, _2 })
    }
}

impl BinaryData for GenericActivableIeParam {
    const TYPE_ID: i16 = 31;
}

#[derive(Debug, Clone, Serialize)]
pub struct GenericActivableIeParamVisualsGroupActionsActions {
    pub action_id: i32,
    pub action_type_id: i32,
    pub criteria: String,
    pub action_params: Vec<String>,
}

impl Decode for GenericActivableIeParamVisualsGroupActionsActions {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let action_id = state.decode()?;
        let action_type_id = state.decode()?;
        let criteria = state.decode()?;
        let action_params = state.decode()?;
        Ok(Self {
            action_id,
            action_type_id,
            criteria,
            action_params,
        })
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct GenericActivableIeParamVisualsGroupActions {
    pub id: i32,
    pub criteria: String,
    pub weight: f32,
    pub actions: Vec<GenericActivableIeParamVisualsGroupActionsActions>,
}

impl Decode for GenericActivableIeParamVisualsGroupActions {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        let criteria = state.decode()?;
        let weight = state.decode()?;
        let actions = state.decode()?;
        Ok(Self {
            id,
            criteria,
            weight,
            actions,
        })
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct GenericActivableIeParamVisuals {
    pub id: i32,
    pub visual_id: i32,
    pub item_consumed: i32,
    pub item_quantity: i32,
    pub do_consume_item: bool,
    pub kama_cost: i32,
    pub distribution_duration: i32,
    pub group_actions: Vec<GenericActivableIeParamVisualsGroupActions>,
}

impl Decode for GenericActivableIeParamVisuals {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        let visual_id = state.decode()?;
        let item_consumed = state.decode()?;
        let item_quantity = state.decode()?;
        let do_consume_item = state.decode()?;
        let kama_cost = state.decode()?;
        let distribution_duration = state.decode()?;
        let group_actions = state.decode()?;
        Ok(Self {
            id,
            visual_id,
            item_consumed,
            item_quantity,
            do_consume_item,
            kama_cost,
            distribution_duration,
            group_actions,
        })
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct GenericActivableIeParam_2 {
    pub _0: i8,
    pub _1: i32,
}

impl Decode for GenericActivableIeParam_2 {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let _0 = state.decode()?;
        let _1 = state.decode()?;
        Ok(Self { _0, _1 })
    }
}
