use crate::BinaryData;
use crate::decode::{Decode, DecodeState};
use std::io;
use std::marker::PhantomData;

#[derive(Debug, Clone, serde::Serialize)]
pub struct Spell {
    pub id: i32,
    pub script_id: i32,
    pub gfx_id: i32,
    pub max_level: i16,
    pub breed_id: i16,
    pub cast_max_per_target: i16,
    pub cast_max_per_turn: f32,
    pub cast_max_per_turn_incr: f32,
    pub cast_min_interval: i16,
    pub test_line_of_sight: bool,
    pub cast_only_line: bool,
    pub cast_only_in_diag: bool,
    pub test_free_cell: bool,
    pub test_not_border_cell: bool,
    pub test_direct_path: bool,
    pub _15: bool,
    pub target_filter: i32,
    pub cast_criterion: String,
    pub _18: i16,
    pub p_a_base: f32,
    pub p_a_inc: f32,
    pub p_m_base: f32,
    pub p_m_inc: f32,
    pub p_w_base: f32,
    pub p_w_inc: f32,
    pub range_max_base: f32,
    pub range_max_inc: f32,
    pub range_min_base: f32,
    pub range_min_level_increment: f32,
    pub max_effect_cap: i16,
    pub element: i16,
    pub xp_gain_percentage: i16,
    pub spell_type: i16,
    pub ui_position: i16,
    pub learn_criteria: String,
    pub _35: String,
    pub passive: i8,
    pub use_automatic_description: bool,
    pub show_in_timeline: bool,
    pub can_cast_when_carrying: bool,
    pub action_on_critical_miss: i8,
    pub spell_cast_range_is_dynamic: bool,
    pub cast_spell_will_break_invisibility: bool,
    pub cast_on_random_cell: bool,
    pub tunnelable: bool,
    pub can_cast_on_caster_cell: bool,
    pub associated_with_item_use: bool,
    pub properties: Vec<i32>,
    pub effect_ids: Vec<i32>,
    pub base_cast_parameters: std::collections::HashMap<i8, SpellBaseCastParameters>,
    pub _50: std::collections::HashMap<i16, Spell_50>,
    pub _51: std::collections::HashMap<String, Spell_51>,
}

impl BinaryData for Spell {
    fn id(_phantom: PhantomData<Self>) -> i32 {
        66
    }
}

impl Decode for Spell {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        let script_id = state.decode()?;
        let gfx_id = state.decode()?;
        let max_level = state.decode()?;
        let breed_id = state.decode()?;
        let cast_max_per_target = state.decode()?;
        let cast_max_per_turn = state.decode()?;
        let cast_max_per_turn_incr = state.decode()?;
        let cast_min_interval = state.decode()?;
        let test_line_of_sight = state.decode()?;
        let cast_only_line = state.decode()?;
        let cast_only_in_diag = state.decode()?;
        let test_free_cell = state.decode()?;
        let test_not_border_cell = state.decode()?;
        let test_direct_path = state.decode()?;
        let _15 = state.decode()?;
        let target_filter = state.decode()?;
        let cast_criterion = state.decode()?;
        let _18 = state.decode()?;
        let p_a_base = state.decode()?;
        let p_a_inc = state.decode()?;
        let p_m_base = state.decode()?;
        let p_m_inc = state.decode()?;
        let p_w_base = state.decode()?;
        let p_w_inc = state.decode()?;
        let range_max_base = state.decode()?;
        let range_max_inc = state.decode()?;
        let range_min_base = state.decode()?;
        let range_min_level_increment = state.decode()?;
        let max_effect_cap = state.decode()?;
        let element = state.decode()?;
        let xp_gain_percentage = state.decode()?;
        let spell_type = state.decode()?;
        let ui_position = state.decode()?;
        let learn_criteria = state.decode()?;
        let _35 = state.decode()?;
        let passive = state.decode()?;
        let use_automatic_description = state.decode()?;
        let show_in_timeline = state.decode()?;
        let can_cast_when_carrying = state.decode()?;
        let action_on_critical_miss = state.decode()?;
        let spell_cast_range_is_dynamic = state.decode()?;
        let cast_spell_will_break_invisibility = state.decode()?;
        let cast_on_random_cell = state.decode()?;
        let tunnelable = state.decode()?;
        let can_cast_on_caster_cell = state.decode()?;
        let associated_with_item_use = state.decode()?;
        let properties = state.decode()?;
        let effect_ids = state.decode()?;
        let base_cast_parameters = state.decode()?;
        let _50 = state.decode()?;
        let _51 = state.decode()?;
        Ok(Spell {
            id,
            script_id,
            gfx_id,
            max_level,
            breed_id,
            cast_max_per_target,
            cast_max_per_turn,
            cast_max_per_turn_incr,
            cast_min_interval,
            test_line_of_sight,
            cast_only_line,
            cast_only_in_diag,
            test_free_cell,
            test_not_border_cell,
            test_direct_path,
            _15,
            target_filter,
            cast_criterion,
            _18,
            p_a_base,
            p_a_inc,
            p_m_base,
            p_m_inc,
            p_w_base,
            p_w_inc,
            range_max_base,
            range_max_inc,
            range_min_base,
            range_min_level_increment,
            max_effect_cap,
            element,
            xp_gain_percentage,
            spell_type,
            ui_position,
            learn_criteria,
            _35,
            passive,
            use_automatic_description,
            show_in_timeline,
            can_cast_when_carrying,
            action_on_critical_miss,
            spell_cast_range_is_dynamic,
            cast_spell_will_break_invisibility,
            cast_on_random_cell,
            tunnelable,
            can_cast_on_caster_cell,
            associated_with_item_use,
            properties,
            effect_ids,
            base_cast_parameters,
            _50,
            _51,
        })
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct Spell_51_1 {
    pub _0: i32,
    pub _1: f32,
}

impl Decode for Spell_51_1 {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let _0 = state.decode()?;
        let _1 = state.decode()?;
        Ok(Spell_51_1 { _0, _1 })
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct Spell_51 {
    pub _0: i32,
    pub _1: std::collections::HashMap<i32, Spell_51_1>,
    pub _2: f32,
    pub _3: f32,
    pub _4: f32,
    pub _5: f32,
    pub _6: f32,
    pub _7: f32,
    pub _8: f32,
    pub _9: f32,
    pub _10: f32,
    pub _11: f32,
    pub _12: bool,
    pub _13: bool,
    pub _14: bool,
    pub _15: bool,
    pub _16: f32,
    pub _17: f32,
    pub _18: f32,
    pub _19: f32,
    pub _20: bool,
    pub _21: bool,
    pub _22: bool,
    pub _23: bool,
}

impl Decode for Spell_51 {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let _0 = state.decode()?;
        let _1 = state.decode()?;
        let _2 = state.decode()?;
        let _3 = state.decode()?;
        let _4 = state.decode()?;
        let _5 = state.decode()?;
        let _6 = state.decode()?;
        let _7 = state.decode()?;
        let _8 = state.decode()?;
        let _9 = state.decode()?;
        let _10 = state.decode()?;
        let _11 = state.decode()?;
        let _12 = state.decode()?;
        let _13 = state.decode()?;
        let _14 = state.decode()?;
        let _15 = state.decode()?;
        let _16 = state.decode()?;
        let _17 = state.decode()?;
        let _18 = state.decode()?;
        let _19 = state.decode()?;
        let _20 = state.decode()?;
        let _21 = state.decode()?;
        let _22 = state.decode()?;
        let _23 = state.decode()?;
        Ok(Spell_51 {
            _0,
            _1,
            _2,
            _3,
            _4,
            _5,
            _6,
            _7,
            _8,
            _9,
            _10,
            _11,
            _12,
            _13,
            _14,
            _15,
            _16,
            _17,
            _18,
            _19,
            _20,
            _21,
            _22,
            _23,
        })
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct Spell_50 {
    pub _0: i16,
    pub _1: Vec<i32>,
}

impl Decode for Spell_50 {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let _0 = state.decode()?;
        let _1 = state.decode()?;
        Ok(Spell_50 { _0, _1 })
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct SpellBaseCastParameters {
    pub base: i32,
    pub increment: f32,
}

impl Decode for SpellBaseCastParameters {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let base = state.decode()?;
        let increment = state.decode()?;
        Ok(SpellBaseCastParameters { base, increment })
    }
}
