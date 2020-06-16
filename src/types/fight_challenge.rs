use crate::BinaryData;
use crate::decode::{Decode, DecodeState};
use std::io;
use std::marker::PhantomData;

#[derive(Debug, Clone, serde::Serialize)]
pub struct FightChallenge {
    pub id: i32,
    pub drop_weight: i32,
    pub drop_criterion: String,
    pub state_id: i32,
    pub listened_effect_success: i32,
    pub listened_effect_failure: i32,
    pub gfx_id: i32,
    pub is_base: bool,
    pub incompatible_challenges: Vec<i32>,
    pub incompatible_monsters: Vec<i32>,
    pub rewards: Vec<FightChallengeRewards>,
}

impl BinaryData for FightChallenge {
    fn id(_phantom: PhantomData<Self>) -> i32 {
        130
    }
}

impl Decode for FightChallenge {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        let drop_weight = state.decode()?;
        let drop_criterion = state.decode()?;
        let state_id = state.decode()?;
        let listened_effect_success = state.decode()?;
        let listened_effect_failure = state.decode()?;
        let gfx_id = state.decode()?;
        let is_base = state.decode()?;
        let incompatible_challenges = state.decode()?;
        let incompatible_monsters = state.decode()?;
        let rewards = state.decode()?;
        Ok(FightChallenge {
            id,
            drop_weight,
            drop_criterion,
            state_id,
            listened_effect_success,
            listened_effect_failure,
            gfx_id,
            is_base,
            incompatible_challenges,
            incompatible_monsters,
            rewards,
        })
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct FightChallengeRewards {
    pub id: i32,
    pub criterion: String,
    pub xp_level: i16,
    pub drop_level: i16,
}

impl Decode for FightChallengeRewards {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        let criterion = state.decode()?;
        let xp_level = state.decode()?;
        let drop_level = state.decode()?;
        Ok(FightChallengeRewards {
            id,
            criterion,
            xp_level,
            drop_level,
        })
    }
}
