use crate::BinaryData;
use crate::decode::{Decode, DecodeState};
use std::io;
use std::marker::PhantomData;

#[derive(Debug, Clone, serde::Serialize)]
pub struct AreaEffect {
    pub id: i32,
    pub script_id: i32,
    pub area_area_id: i32,
    pub max_execution_count: i32,
    pub targets_to_show: i32,
    pub can_be_targeted: bool,
    pub obstacle_for_a_i: bool,
    pub should_stop_movement: bool,
    pub can_be_targeted_by_a_i: bool,
    pub can_be_destroyed: bool,
    pub kind: String,
    pub area_area_params: Vec<i32>,
    pub application_triggers: Vec<i32>,
    pub unapplication_triggers: Vec<i32>,
    pub destruction_triggers: Vec<i32>,
    pub deactivation_delay: Vec<f32>,
    pub params: Vec<f32>,
    pub properties: Vec<i32>,
    pub effect_ids: Vec<i32>,
    pub area_gfx: String,
    pub area_cell_gfx: String,
    pub aps: String,
    pub cell_aps: String,
    pub max_level: i32,
}

impl BinaryData for AreaEffect {
    fn id(_phantom: PhantomData<Self>) -> i32 {
        4
    }
}

impl Decode for AreaEffect {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        let script_id = state.decode()?;
        let area_area_id = state.decode()?;
        let max_execution_count = state.decode()?;
        let targets_to_show = state.decode()?;
        let can_be_targeted = state.decode()?;
        let obstacle_for_a_i = state.decode()?;
        let should_stop_movement = state.decode()?;
        let can_be_targeted_by_a_i = state.decode()?;
        let can_be_destroyed = state.decode()?;
        let kind = state.decode()?;
        let area_area_params = state.decode()?;
        let application_triggers = state.decode()?;
        let unapplication_triggers = state.decode()?;
        let destruction_triggers = state.decode()?;
        let deactivation_delay = state.decode()?;
        let params = state.decode()?;
        let properties = state.decode()?;
        let effect_ids = state.decode()?;
        let area_gfx = state.decode()?;
        let area_cell_gfx = state.decode()?;
        let aps = state.decode()?;
        let cell_aps = state.decode()?;
        let max_level = state.decode()?;
        Ok(AreaEffect {
            id,
            script_id,
            area_area_id,
            max_execution_count,
            targets_to_show,
            can_be_targeted,
            obstacle_for_a_i,
            should_stop_movement,
            can_be_targeted_by_a_i,
            can_be_destroyed,
            kind,
            area_area_params,
            application_triggers,
            unapplication_triggers,
            destruction_triggers,
            deactivation_delay,
            params,
            properties,
            effect_ids,
            area_gfx,
            area_cell_gfx,
            aps,
            cell_aps,
            max_level,
        })
    }
}
