use crate::BinaryData;
use crate::decode::{Decode, DecodeState};
use std::io;
use std::marker::PhantomData;

#[derive(Debug, Clone, serde::Serialize)]
pub struct State {
    pub id: i32,
    pub max_level: i16,
    pub end_trigger: Vec<i32>,
    pub duration: Vec<i32>,
    pub _4: Vec<i32>,
    pub ends_at_end_of_turn: bool,
    pub is_duration_in_full_turns: bool,
    pub in_turn_in_fight: bool,
    pub is_replacable: bool,
    pub hmi_actions: String,
    pub apply_criterion: String,
    pub is_cumulable: bool,
    pub duration_in_caster_turn: bool,
    pub duration_in_real_time: bool,
    pub effect_ids: Vec<i32>,
    pub state_immunities: Vec<i32>,
    pub state_should_be_saved: bool,
    pub decursable: bool,
    pub state_type: i8,
    pub state_power_type: i8,
    pub is_reapply_even_at_max_level: bool,
    pub timeline_visible: bool,
    pub _22: String,
    pub display_caster_name: bool,
    pub _24: i16,
}

impl BinaryData for State {
    fn id(_phantom: PhantomData<Self>) -> i32 {
        67
    }
}

impl Decode for State {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        let max_level = state.decode()?;
        let end_trigger = state.decode()?;
        let duration = state.decode()?;
        let _4 = state.decode()?;
        let ends_at_end_of_turn = state.decode()?;
        let is_duration_in_full_turns = state.decode()?;
        let in_turn_in_fight = state.decode()?;
        let is_replacable = state.decode()?;
        let hmi_actions = state.decode()?;
        let apply_criterion = state.decode()?;
        let is_cumulable = state.decode()?;
        let duration_in_caster_turn = state.decode()?;
        let duration_in_real_time = state.decode()?;
        let effect_ids = state.decode()?;
        let state_immunities = state.decode()?;
        let state_should_be_saved = state.decode()?;
        let decursable = state.decode()?;
        let state_type = state.decode()?;
        let state_power_type = state.decode()?;
        let is_reapply_even_at_max_level = state.decode()?;
        let timeline_visible = state.decode()?;
        let _22 = state.decode()?;
        let display_caster_name = state.decode()?;
        let _24 = state.decode()?;
        Ok(State {
            id,
            max_level,
            end_trigger,
            duration,
            _4,
            ends_at_end_of_turn,
            is_duration_in_full_turns,
            in_turn_in_fight,
            is_replacable,
            hmi_actions,
            apply_criterion,
            is_cumulable,
            duration_in_caster_turn,
            duration_in_real_time,
            effect_ids,
            state_immunities,
            state_should_be_saved,
            decursable,
            state_type,
            state_power_type,
            is_reapply_even_at_max_level,
            timeline_visible,
            _22,
            display_caster_name,
            _24,
        })
    }
}
