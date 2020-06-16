use crate::BinaryData;
use crate::decode::{Decode, DecodeState};
use std::io;
use std::marker::PhantomData;

#[derive(Debug, Clone, serde::Serialize)]
pub struct ArcadeDungeon {
    pub id: i32,
    pub world_id: i16,
    pub challenges: Vec<ArcadeDungeonChallenges>,
    pub rewards_list: Vec<ArcadeDungeonRewardsList>,
    pub ranks: Vec<ArcadeDungeonRanks>,
    pub score_round_base: i32,
    pub score_round_incr: i32,
}

impl BinaryData for ArcadeDungeon {
    fn id(_phantom: PhantomData<Self>) -> i32 {
        87
    }
}

impl Decode for ArcadeDungeon {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        let world_id = state.decode()?;
        let challenges = state.decode()?;
        let rewards_list = state.decode()?;
        let ranks = state.decode()?;
        let score_round_base = state.decode()?;
        let score_round_incr = state.decode()?;
        Ok(ArcadeDungeon {
            id,
            world_id,
            challenges,
            rewards_list,
            ranks,
            score_round_base,
            score_round_incr,
        })
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct ArcadeDungeonRanks {
    pub level_order: i32,
    pub score_min_d: i32,
    pub score_min_c: i32,
    pub score_min_b: i32,
    pub score_min_a: i32,
}

impl Decode for ArcadeDungeonRanks {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let level_order = state.decode()?;
        let score_min_d = state.decode()?;
        let score_min_c = state.decode()?;
        let score_min_b = state.decode()?;
        let score_min_a = state.decode()?;
        Ok(ArcadeDungeonRanks {
            level_order,
            score_min_d,
            score_min_c,
            score_min_b,
            score_min_a,
        })
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct ArcadeDungeonRewardsListRewards {
    pub score_min: i32,
    pub item_id: i32,
    pub xp_value: i32,
    pub rank_needed: i8,
}

impl Decode for ArcadeDungeonRewardsListRewards {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let score_min = state.decode()?;
        let item_id = state.decode()?;
        let xp_value = state.decode()?;
        let rank_needed = state.decode()?;
        Ok(ArcadeDungeonRewardsListRewards {
            score_min,
            item_id,
            xp_value,
            rank_needed,
        })
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct ArcadeDungeonRewardsList {
    pub id: i32,
    pub level_order: i32,
    pub rewards: Vec<ArcadeDungeonRewardsListRewards>,
}

impl Decode for ArcadeDungeonRewardsList {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        let level_order = state.decode()?;
        let rewards = state.decode()?;
        Ok(ArcadeDungeonRewardsList {
            id,
            level_order,
            rewards,
        })
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct ArcadeDungeonChallenges {
    pub id: i32,
    pub ratio: f32,
}

impl Decode for ArcadeDungeonChallenges {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        let ratio = state.decode()?;
        Ok(ArcadeDungeonChallenges { id, ratio })
    }
}
