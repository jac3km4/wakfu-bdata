use crate::BinaryData;
use crate::decode::{Decode, DecodeState};
use std::io;
use std::marker::PhantomData;

#[derive(Debug, Clone, serde::Serialize)]
pub struct Item {
    pub id: i32,
    pub item_set_id: i16,
    pub gfx_id: i32,
    pub female_gfx_id: i32,
    pub floor_gfx_id: i32,
    pub level: i16,
    pub criteria: Vec<String>,
    pub item_type_id: i32,
    pub max_stack_height: i16,
    pub use_cost_a_p: i8,
    pub use_cost_m_p: i8,
    pub use_cost_f_p: i8,
    pub use_range_min: i32,
    pub use_range_max: i32,
    pub use_test_free_cell: bool,
    pub use_test_not_border_cell: bool,
    pub use_test_los: bool,
    pub use_test_only_line: bool,
    pub item_rarity: i16,
    pub item_bind_type: i8,
    pub generation_type: String,
    pub item_properties: Vec<i32>,
    pub item_action_visual: i8,
    pub world_usage_target: i8,
    pub gem_element_type: i8,
    pub gem_num: i8,
    pub _26: f32,
    pub _27: f32,
    pub _28: i8,
    pub effect_ids: Vec<i32>,
    pub actions: Vec<ItemActions>,
    pub _31: Option<Item_31>,
    pub _32: Option<Item_32>,
}

impl BinaryData for Item {
    fn id(_phantom: PhantomData<Self>) -> i32 {
        35
    }
}

impl Decode for Item {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        let item_set_id = state.decode()?;
        let gfx_id = state.decode()?;
        let female_gfx_id = state.decode()?;
        let floor_gfx_id = state.decode()?;
        let level = state.decode()?;
        let criteria = state.decode()?;
        let item_type_id = state.decode()?;
        let max_stack_height = state.decode()?;
        let use_cost_a_p = state.decode()?;
        let use_cost_m_p = state.decode()?;
        let use_cost_f_p = state.decode()?;
        let use_range_min = state.decode()?;
        let use_range_max = state.decode()?;
        let use_test_free_cell = state.decode()?;
        let use_test_not_border_cell = state.decode()?;
        let use_test_los = state.decode()?;
        let use_test_only_line = state.decode()?;
        let item_rarity = state.decode()?;
        let item_bind_type = state.decode()?;
        let generation_type = state.decode()?;
        let item_properties = state.decode()?;
        let item_action_visual = state.decode()?;
        let world_usage_target = state.decode()?;
        let gem_element_type = state.decode()?;
        let gem_num = state.decode()?;
        let _26 = state.decode()?;
        let _27 = state.decode()?;
        let _28 = state.decode()?;
        let effect_ids = state.decode()?;
        let actions = state.decode()?;
        let _31 = state.decode()?;
        let _32 = state.decode()?;
        Ok(Item {
            id,
            item_set_id,
            gfx_id,
            female_gfx_id,
            floor_gfx_id,
            level,
            criteria,
            item_type_id,
            max_stack_height,
            use_cost_a_p,
            use_cost_m_p,
            use_cost_f_p,
            use_range_min,
            use_range_max,
            use_test_free_cell,
            use_test_not_border_cell,
            use_test_los,
            use_test_only_line,
            item_rarity,
            item_bind_type,
            generation_type,
            item_properties,
            item_action_visual,
            world_usage_target,
            gem_element_type,
            gem_num,
            _26,
            _27,
            _28,
            effect_ids,
            actions,
            _31,
            _32,
        })
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct Item_32 {
    pub _0: i32,
    pub _1: Vec<i8>,
    pub _2: bool,
    pub _3: bool,
}

impl Decode for Item_32 {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let _0 = state.decode()?;
        let _1 = state.decode()?;
        let _2 = state.decode()?;
        let _3 = state.decode()?;
        Ok(Item_32 { _0, _1, _2, _3 })
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct Item_31 {
    pub _0: i8,
    pub _1: Vec<i8>,
    pub _2: Vec<i32>,
    pub _3: Vec<i32>,
    pub _4: bool,
}

impl Decode for Item_31 {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let _0 = state.decode()?;
        let _1 = state.decode()?;
        let _2 = state.decode()?;
        let _3 = state.decode()?;
        let _4 = state.decode()?;
        Ok(Item_31 { _0, _1, _2, _3, _4 })
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct ItemActions {
    pub action_id: i32,
    pub action_type_id: i32,
    pub consume_item_on_action: bool,
    pub client_only: bool,
    pub stop_movement: bool,
    pub has_script: bool,
    pub criteria: String,
    pub action_params: Vec<String>,
    pub action_script_params: Vec<String>,
}

impl Decode for ItemActions {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let action_id = state.decode()?;
        let action_type_id = state.decode()?;
        let consume_item_on_action = state.decode()?;
        let client_only = state.decode()?;
        let stop_movement = state.decode()?;
        let has_script = state.decode()?;
        let criteria = state.decode()?;
        let action_params = state.decode()?;
        let action_script_params = state.decode()?;
        Ok(ItemActions {
            action_id,
            action_type_id,
            consume_item_on_action,
            client_only,
            stop_movement,
            has_script,
            criteria,
            action_params,
            action_script_params,
        })
    }
}
