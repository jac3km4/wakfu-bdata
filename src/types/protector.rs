use crate::BinaryData;
use crate::decode::{Decode, DecodeState};
use std::io;
use std::marker::PhantomData;

#[derive(Debug, Clone, serde::Serialize)]
pub struct Protector {
    pub protector_id: i32,
    pub monster_id: i32,
    pub buff_list_id: i32,
    pub buff_list_id_to_buy: i32,
    pub scenario_loot_list_id: i32,
    pub scenario_loot_list_id_to_buy: i32,
    pub scenario_loot_list_id_chaos: i32,
    pub scenario_loot_list_id_ecosystem: i32,
    pub climate_list_id_to_buy: i32,
    pub nation_id: i32,
    pub territory: i32,
    pub fight_stake: i32,
    pub position_z: i16,
    pub craft_learnt: Vec<i32>,
    pub secrets: Vec<ProtectorSecrets>,
    pub fauna_will: Vec<ProtectorFaunaWill>,
    pub flora_will: Vec<ProtectorFloraWill>,
}

impl BinaryData for Protector {
    fn id(_phantom: PhantomData<Self>) -> i32 {
        54
    }
}

impl Decode for Protector {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let protector_id = state.decode()?;
        let monster_id = state.decode()?;
        let buff_list_id = state.decode()?;
        let buff_list_id_to_buy = state.decode()?;
        let scenario_loot_list_id = state.decode()?;
        let scenario_loot_list_id_to_buy = state.decode()?;
        let scenario_loot_list_id_chaos = state.decode()?;
        let scenario_loot_list_id_ecosystem = state.decode()?;
        let climate_list_id_to_buy = state.decode()?;
        let nation_id = state.decode()?;
        let territory = state.decode()?;
        let fight_stake = state.decode()?;
        let position_z = state.decode()?;
        let craft_learnt = state.decode()?;
        let secrets = state.decode()?;
        let fauna_will = state.decode()?;
        let flora_will = state.decode()?;
        Ok(Protector {
            protector_id,
            monster_id,
            buff_list_id,
            buff_list_id_to_buy,
            scenario_loot_list_id,
            scenario_loot_list_id_to_buy,
            scenario_loot_list_id_chaos,
            scenario_loot_list_id_ecosystem,
            climate_list_id_to_buy,
            nation_id,
            territory,
            fight_stake,
            position_z,
            craft_learnt,
            secrets,
            fauna_will,
            flora_will,
        })
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct ProtectorFloraWill {
    pub kind_id: i32,
    pub min: i16,
    pub max: i16,
}

impl Decode for ProtectorFloraWill {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let kind_id = state.decode()?;
        let min = state.decode()?;
        let max = state.decode()?;
        Ok(ProtectorFloraWill { kind_id, min, max })
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct ProtectorFaunaWill {
    pub kind_id: i32,
    pub min: i16,
    pub max: i16,
}

impl Decode for ProtectorFaunaWill {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let kind_id = state.decode()?;
        let min = state.decode()?;
        let max = state.decode()?;
        Ok(ProtectorFaunaWill { kind_id, min, max })
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct ProtectorSecrets {
    pub id: i32,
    pub achievement_goal_id: i32,
    pub secret_gfx_id: i32,
    pub discovered_gfx_id: i32,
}

impl Decode for ProtectorSecrets {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        let achievement_goal_id = state.decode()?;
        let secret_gfx_id = state.decode()?;
        let discovered_gfx_id = state.decode()?;
        Ok(ProtectorSecrets {
            id,
            achievement_goal_id,
            secret_gfx_id,
            discovered_gfx_id,
        })
    }
}
