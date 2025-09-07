use std::io;

use serde::Serialize;

use crate::data::BinaryData;
use crate::decode::{Decode, DecodeState};

#[derive(Debug, Clone, Serialize)]
pub struct StaticEffect {
    pub effect_id: i32,
    pub action_id: i32,
    pub parent_id: i32,
    pub area_ordering_method: i16,
    pub area_size: Vec<i32>,
    pub area_shape: i16,
    pub empty_cells_area_ordering_method: i16,
    pub empty_cells_area_size: Vec<i32>,
    pub empty_cells_area_shape: i16,
    pub triggers_before_computation: Vec<i32>,
    pub triggers_before_execution: Vec<i32>,
    pub triggers_for_unapplication: Vec<i32>,
    pub triggers_after_execution: Vec<i32>,
    pub triggers_after_all_executions: Vec<i32>,
    pub triggers_not_related_to_executions: Vec<i32>,
    pub triggers_additionnal: Vec<i32>,
    pub critical_state: String,
    pub target_validator: Vec<i64>,
    pub affected_by_localisation: bool,
    pub duration_base: i32,
    pub duration_inc: f32,
    pub ends_at_end_of_turn: bool,
    pub is_duration_in_full_turns: bool,
    pub apply_delay_base: i16,
    pub apply_delay_increment: f32,
    pub params: Vec<f32>,
    pub probability_base: f32,
    pub probability_inc: f32,
    pub trigger_listener_type: i8,
    pub trigger_target_type: i8,
    pub trigger_caster_type: i8,
    pub _31: i32,
    pub store_on_self: bool,
    pub max_execution: i16,
    pub max_execution_incr: f32,
    pub max_target_count: i8,
    pub is_fight_effect: bool,
    pub hmi_action: String,
    pub container_min_level: i16,
    pub container_max_level: i16,
    pub effect_criterion: String,
    pub effect_parent_type: String,
    pub effect_container_type: String,
    pub dont_trigger_anything: bool,
    pub is_personal: bool,
    pub is_decursable: bool,
    pub notify_in_chat_for_caster: bool,
    pub notify_in_chat_for_target: bool,
    pub notify_in_chat_with_caster_name: bool,
    pub script_file_id: i32,
    pub duration_in_caster_turn: bool,
    pub effect_properties: Vec<i32>,
    pub display_in_spell_description: bool,
    pub display_in_state_bar: bool,
    pub recompute_area_of_effect_display: bool,
    pub is_in_turn_in_fight: bool,
    pub notify_in_chat: bool,
}

impl Decode for StaticEffect {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let effect_id = state.decode()?;
        let action_id = state.decode()?;
        let parent_id = state.decode()?;
        let area_ordering_method = state.decode()?;
        let area_size = state.decode()?;
        let area_shape = state.decode()?;
        let empty_cells_area_ordering_method = state.decode()?;
        let empty_cells_area_size = state.decode()?;
        let empty_cells_area_shape = state.decode()?;
        let triggers_before_computation = state.decode()?;
        let triggers_before_execution = state.decode()?;
        let triggers_for_unapplication = state.decode()?;
        let triggers_after_execution = state.decode()?;
        let triggers_after_all_executions = state.decode()?;
        let triggers_not_related_to_executions = state.decode()?;
        let triggers_additionnal = state.decode()?;
        let critical_state = state.decode()?;
        let target_validator = state.decode()?;
        let affected_by_localisation = state.decode()?;
        let duration_base = state.decode()?;
        let duration_inc = state.decode()?;
        let ends_at_end_of_turn = state.decode()?;
        let is_duration_in_full_turns = state.decode()?;
        let apply_delay_base = state.decode()?;
        let apply_delay_increment = state.decode()?;
        let params = state.decode()?;
        let probability_base = state.decode()?;
        let probability_inc = state.decode()?;
        let trigger_listener_type = state.decode()?;
        let trigger_target_type = state.decode()?;
        let trigger_caster_type = state.decode()?;
        let _31 = state.decode()?;
        let store_on_self = state.decode()?;
        let max_execution = state.decode()?;
        let max_execution_incr = state.decode()?;
        let max_target_count = state.decode()?;
        let is_fight_effect = state.decode()?;
        let hmi_action = state.decode()?;
        let container_min_level = state.decode()?;
        let container_max_level = state.decode()?;
        let effect_criterion = state.decode()?;
        let effect_parent_type = state.decode()?;
        let effect_container_type = state.decode()?;
        let dont_trigger_anything = state.decode()?;
        let is_personal = state.decode()?;
        let is_decursable = state.decode()?;
        let notify_in_chat_for_caster = state.decode()?;
        let notify_in_chat_for_target = state.decode()?;
        let notify_in_chat_with_caster_name = state.decode()?;
        let script_file_id = state.decode()?;
        let duration_in_caster_turn = state.decode()?;
        let effect_properties = state.decode()?;
        let display_in_spell_description = state.decode()?;
        let display_in_state_bar = state.decode()?;
        let recompute_area_of_effect_display = state.decode()?;
        let is_in_turn_in_fight = state.decode()?;
        let notify_in_chat = state.decode()?;
        Ok(Self {
            effect_id,
            action_id,
            parent_id,
            area_ordering_method,
            area_size,
            area_shape,
            empty_cells_area_ordering_method,
            empty_cells_area_size,
            empty_cells_area_shape,
            triggers_before_computation,
            triggers_before_execution,
            triggers_for_unapplication,
            triggers_after_execution,
            triggers_after_all_executions,
            triggers_not_related_to_executions,
            triggers_additionnal,
            critical_state,
            target_validator,
            affected_by_localisation,
            duration_base,
            duration_inc,
            ends_at_end_of_turn,
            is_duration_in_full_turns,
            apply_delay_base,
            apply_delay_increment,
            params,
            probability_base,
            probability_inc,
            trigger_listener_type,
            trigger_target_type,
            trigger_caster_type,
            _31,
            store_on_self,
            max_execution,
            max_execution_incr,
            max_target_count,
            is_fight_effect,
            hmi_action,
            container_min_level,
            container_max_level,
            effect_criterion,
            effect_parent_type,
            effect_container_type,
            dont_trigger_anything,
            is_personal,
            is_decursable,
            notify_in_chat_for_caster,
            notify_in_chat_for_target,
            notify_in_chat_with_caster_name,
            script_file_id,
            duration_in_caster_turn,
            effect_properties,
            display_in_spell_description,
            display_in_state_bar,
            recompute_area_of_effect_display,
            is_in_turn_in_fight,
            notify_in_chat,
        })
    }
}

impl BinaryData for StaticEffect {
    const TYPE_ID: i16 = 68;
}
