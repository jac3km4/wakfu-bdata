use std::io;

use serde::Serialize;

use crate::data::BinaryData;
use crate::decode::{Decode, DecodeState};

#[derive(Debug, Clone, Serialize)]
pub struct Monster {
    pub id: i32,
    pub family_id: i32,
    pub level_min: i16,
    pub level_max: i16,
    pub xp_multiplicator: f32,
    pub arcade_points_multiplicator: f32,
    pub base_hp: i32,
    pub base_wp: i32,
    pub base_ap: i32,
    pub base_mp: i32,
    pub base_range: i32,
    pub base_init: i32,
    pub base_perception: i32,
    pub base_parade: i32,
    pub base_will: i32,
    pub base_critical_hit: i32,
    pub base_time_before_death: i32,
    pub _17: i32,
    pub _18: i32,
    pub hp_inc: f32,
    pub wp_inc: f32,
    pub ap_inc: f32,
    pub mp_inc: f32,
    pub range_inc: f32,
    pub init_inc: f32,
    pub perception_inc: f32,
    pub parade_inc: f32,
    pub will_inc: f32,
    pub critical_hit_inc: f32,
    pub _29: f32,
    pub _30: f32,
    pub base_healing_bonus: i32,
    pub base_summon_bonus: i32,
    pub base_mechanics_bonus: i32,
    pub base_tackle_bonus: i32,
    pub base_fire_damage_bonus: i32,
    pub base_water_damage_bonus: i32,
    pub base_earth_damage_bonus: i32,
    pub base_wind_damage_bonus: i32,
    pub base_fire_resistance: i32,
    pub base_water_resistance: i32,
    pub base_earth_resistance: i32,
    pub base_wind_resistance: i32,
    pub base_tackle_resistance: i32,
    pub base_ap_loss_resistance: i32,
    pub base_pm_loss_resistance: i32,
    pub base_wp_loss_resistance: i32,
    pub healing_bonus_inc: f32,
    pub tackle_bonus_inc: f32,
    pub fire_damage_bonus_inc: f32,
    pub water_damage_bonus_inc: f32,
    pub earth_damage_bonus_inc: f32,
    pub wind_damage_bonus_inc: f32,
    pub fire_resistance_inc: f32,
    pub water_resistance_inc: f32,
    pub earth_resistance_inc: f32,
    pub wind_resistance_inc: f32,
    pub tackle_resistance_inc: f32,
    pub ap_loss_resistance_inc: f32,
    pub pm_loss_resistance_inc: f32,
    pub wp_loss_resistance_inc: f32,
    pub has_dead_evolution: bool,
    pub npc_rank: i16,
    pub agro_radius: i16,
    pub sight_radius: i16,
    pub radius_size: i32,
    pub fight_properties: Vec<i32>,
    pub world_properties: Vec<i32>,
    pub natural_states: Vec<i16>,
    pub spells_id_and_level: Vec<MonsterSpellsIdAndLevel>,
    pub family_rank: i8,
    pub walking_speed: i16,
    pub running_speed: i16,
    pub running_radius_in_world: i16,
    pub running_radius_in_fight: i16,
    pub monster_action_data: Vec<MonsterMonsterActionData>,
    pub monster_collect_action_data: Vec<MonsterMonsterCollectActionData>,
    pub monster_behaviour_data: Vec<MonsterMonsterBehaviourData>,
    pub monster_evolution_data: Vec<MonsterMonsterEvolutionData>,
    pub required_leadership_points: i32,
    pub alignment_id: i16,
    pub timeline_buff_id: i32,
    pub monster_height: i32,
    pub defeat_script_id: i32,
    pub gfx_equipment: Vec<i32>,
    pub special_gfx_equipement: Vec<MonsterSpecialGfxEquipement>,
    pub special_gfx_color: Vec<MonsterSpecialGfxColor>,
    pub special_gfx_anim: Vec<MonsterSpecialGfxAnim>,
    pub gfx: i32,
    pub timeline_gfx: i32,
    pub _90: String,
}

impl Decode for Monster {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        let family_id = state.decode()?;
        let level_min = state.decode()?;
        let level_max = state.decode()?;
        let xp_multiplicator = state.decode()?;
        let arcade_points_multiplicator = state.decode()?;
        let base_hp = state.decode()?;
        let base_wp = state.decode()?;
        let base_ap = state.decode()?;
        let base_mp = state.decode()?;
        let base_range = state.decode()?;
        let base_init = state.decode()?;
        let base_perception = state.decode()?;
        let base_parade = state.decode()?;
        let base_will = state.decode()?;
        let base_critical_hit = state.decode()?;
        let base_time_before_death = state.decode()?;
        let _17 = state.decode()?;
        let _18 = state.decode()?;
        let hp_inc = state.decode()?;
        let wp_inc = state.decode()?;
        let ap_inc = state.decode()?;
        let mp_inc = state.decode()?;
        let range_inc = state.decode()?;
        let init_inc = state.decode()?;
        let perception_inc = state.decode()?;
        let parade_inc = state.decode()?;
        let will_inc = state.decode()?;
        let critical_hit_inc = state.decode()?;
        let _29 = state.decode()?;
        let _30 = state.decode()?;
        let base_healing_bonus = state.decode()?;
        let base_summon_bonus = state.decode()?;
        let base_mechanics_bonus = state.decode()?;
        let base_tackle_bonus = state.decode()?;
        let base_fire_damage_bonus = state.decode()?;
        let base_water_damage_bonus = state.decode()?;
        let base_earth_damage_bonus = state.decode()?;
        let base_wind_damage_bonus = state.decode()?;
        let base_fire_resistance = state.decode()?;
        let base_water_resistance = state.decode()?;
        let base_earth_resistance = state.decode()?;
        let base_wind_resistance = state.decode()?;
        let base_tackle_resistance = state.decode()?;
        let base_ap_loss_resistance = state.decode()?;
        let base_pm_loss_resistance = state.decode()?;
        let base_wp_loss_resistance = state.decode()?;
        let healing_bonus_inc = state.decode()?;
        let tackle_bonus_inc = state.decode()?;
        let fire_damage_bonus_inc = state.decode()?;
        let water_damage_bonus_inc = state.decode()?;
        let earth_damage_bonus_inc = state.decode()?;
        let wind_damage_bonus_inc = state.decode()?;
        let fire_resistance_inc = state.decode()?;
        let water_resistance_inc = state.decode()?;
        let earth_resistance_inc = state.decode()?;
        let wind_resistance_inc = state.decode()?;
        let tackle_resistance_inc = state.decode()?;
        let ap_loss_resistance_inc = state.decode()?;
        let pm_loss_resistance_inc = state.decode()?;
        let wp_loss_resistance_inc = state.decode()?;
        let has_dead_evolution = state.decode()?;
        let npc_rank = state.decode()?;
        let agro_radius = state.decode()?;
        let sight_radius = state.decode()?;
        let radius_size = state.decode()?;
        let fight_properties = state.decode()?;
        let world_properties = state.decode()?;
        let natural_states = state.decode()?;
        let spells_id_and_level = state.decode()?;
        let family_rank = state.decode()?;
        let walking_speed = state.decode()?;
        let running_speed = state.decode()?;
        let running_radius_in_world = state.decode()?;
        let running_radius_in_fight = state.decode()?;
        let monster_action_data = state.decode()?;
        let monster_collect_action_data = state.decode()?;
        let monster_behaviour_data = state.decode()?;
        let monster_evolution_data = state.decode()?;
        let required_leadership_points = state.decode()?;
        let alignment_id = state.decode()?;
        let timeline_buff_id = state.decode()?;
        let monster_height = state.decode()?;
        let defeat_script_id = state.decode()?;
        let gfx_equipment = state.decode()?;
        let special_gfx_equipement = state.decode()?;
        let special_gfx_color = state.decode()?;
        let special_gfx_anim = state.decode()?;
        let gfx = state.decode()?;
        let timeline_gfx = state.decode()?;
        let _90 = state.decode()?;
        Ok(Self {
            id,
            family_id,
            level_min,
            level_max,
            xp_multiplicator,
            arcade_points_multiplicator,
            base_hp,
            base_wp,
            base_ap,
            base_mp,
            base_range,
            base_init,
            base_perception,
            base_parade,
            base_will,
            base_critical_hit,
            base_time_before_death,
            _17,
            _18,
            hp_inc,
            wp_inc,
            ap_inc,
            mp_inc,
            range_inc,
            init_inc,
            perception_inc,
            parade_inc,
            will_inc,
            critical_hit_inc,
            _29,
            _30,
            base_healing_bonus,
            base_summon_bonus,
            base_mechanics_bonus,
            base_tackle_bonus,
            base_fire_damage_bonus,
            base_water_damage_bonus,
            base_earth_damage_bonus,
            base_wind_damage_bonus,
            base_fire_resistance,
            base_water_resistance,
            base_earth_resistance,
            base_wind_resistance,
            base_tackle_resistance,
            base_ap_loss_resistance,
            base_pm_loss_resistance,
            base_wp_loss_resistance,
            healing_bonus_inc,
            tackle_bonus_inc,
            fire_damage_bonus_inc,
            water_damage_bonus_inc,
            earth_damage_bonus_inc,
            wind_damage_bonus_inc,
            fire_resistance_inc,
            water_resistance_inc,
            earth_resistance_inc,
            wind_resistance_inc,
            tackle_resistance_inc,
            ap_loss_resistance_inc,
            pm_loss_resistance_inc,
            wp_loss_resistance_inc,
            has_dead_evolution,
            npc_rank,
            agro_radius,
            sight_radius,
            radius_size,
            fight_properties,
            world_properties,
            natural_states,
            spells_id_and_level,
            family_rank,
            walking_speed,
            running_speed,
            running_radius_in_world,
            running_radius_in_fight,
            monster_action_data,
            monster_collect_action_data,
            monster_behaviour_data,
            monster_evolution_data,
            required_leadership_points,
            alignment_id,
            timeline_buff_id,
            monster_height,
            defeat_script_id,
            gfx_equipment,
            special_gfx_equipement,
            special_gfx_color,
            special_gfx_anim,
            gfx,
            timeline_gfx,
            _90,
        })
    }
}

impl BinaryData for Monster {
    const TYPE_ID: i16 = 42;
}

#[derive(Debug, Clone, Serialize)]
pub struct MonsterSpellsIdAndLevel {
    pub id: i32,
    pub level: i32,
}

impl Decode for MonsterSpellsIdAndLevel {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        let level = state.decode()?;
        Ok(Self { id, level })
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct MonsterMonsterActionData {
    pub id: i32,
    pub kind: i8,
    pub criteria: String,
    pub criteria_on_npc: bool,
    pub move_to_monster_before_interact_with_him: bool,
    pub duration: i32,
    pub show_progress_bar: bool,
    pub visual_id: i32,
    pub script_id: i32,
}

impl Decode for MonsterMonsterActionData {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        let kind = state.decode()?;
        let criteria = state.decode()?;
        let criteria_on_npc = state.decode()?;
        let move_to_monster_before_interact_with_him = state.decode()?;
        let duration = state.decode()?;
        let show_progress_bar = state.decode()?;
        let visual_id = state.decode()?;
        let script_id = state.decode()?;
        Ok(Self {
            id,
            kind,
            criteria,
            criteria_on_npc,
            move_to_monster_before_interact_with_him,
            duration,
            show_progress_bar,
            visual_id,
            script_id,
        })
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct MonsterMonsterCollectActionData {
    pub id: i32,
    pub skill_id: i32,
    pub skill_level_required: i32,
    pub skill_visual_feedback_id: i32,
    pub criteria: String,
    pub xp_factor: f32,
    pub collect_loot_list_id: i32,
    pub duration: i32,
    pub collect_item_id: i32,
    pub loot_list: Vec<i32>,
    pub display_in_craft_dialog: bool,
}

impl Decode for MonsterMonsterCollectActionData {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        let skill_id = state.decode()?;
        let skill_level_required = state.decode()?;
        let skill_visual_feedback_id = state.decode()?;
        let criteria = state.decode()?;
        let xp_factor = state.decode()?;
        let collect_loot_list_id = state.decode()?;
        let duration = state.decode()?;
        let collect_item_id = state.decode()?;
        let loot_list = state.decode()?;
        let display_in_craft_dialog = state.decode()?;
        Ok(Self {
            id,
            skill_id,
            skill_level_required,
            skill_visual_feedback_id,
            criteria,
            xp_factor,
            collect_loot_list_id,
            duration,
            collect_item_id,
            loot_list,
            display_in_craft_dialog,
        })
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct MonsterMonsterBehaviourData {
    pub id: i32,
    pub kind: i32,
    pub script_id: i32,
    pub needs_to_wait_path_end: bool,
}

impl Decode for MonsterMonsterBehaviourData {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        let kind = state.decode()?;
        let script_id = state.decode()?;
        let needs_to_wait_path_end = state.decode()?;
        Ok(Self {
            id,
            kind,
            script_id,
            needs_to_wait_path_end,
        })
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct MonsterMonsterEvolutionData {
    pub id: i32,
    pub breed_id: i32,
    pub script_id: i32,
}

impl Decode for MonsterMonsterEvolutionData {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        let breed_id = state.decode()?;
        let script_id = state.decode()?;
        Ok(Self {
            id,
            breed_id,
            script_id,
        })
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct MonsterSpecialGfxEquipement {
    pub file_id: String,
    pub parts: Vec<String>,
}

impl Decode for MonsterSpecialGfxEquipement {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let file_id = state.decode()?;
        let parts = state.decode()?;
        Ok(Self { file_id, parts })
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct MonsterSpecialGfxColor {
    pub part_index: i32,
    pub color: i32,
}

impl Decode for MonsterSpecialGfxColor {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let part_index = state.decode()?;
        let color = state.decode()?;
        Ok(Self { part_index, color })
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct MonsterSpecialGfxAnim {
    pub key: i8,
    pub anim_name: String,
}

impl Decode for MonsterSpecialGfxAnim {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let key = state.decode()?;
        let anim_name = state.decode()?;
        Ok(Self { key, anim_name })
    }
}
