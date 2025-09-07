use std::io;

use serde::Serialize;

use crate::data::BinaryData;
use crate::decode::{Decode, DecodeState};

#[derive(Debug, Clone, Serialize)]
pub struct FightChallengeMonster {
    pub id: i32,
    pub random_rolls: i16,
    pub forced_rolls: i16,
    pub forced_challenges: Vec<i32>,
}

impl Decode for FightChallengeMonster {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        let random_rolls = state.decode()?;
        let forced_rolls = state.decode()?;
        let forced_challenges = state.decode()?;
        Ok(Self {
            id,
            random_rolls,
            forced_rolls,
            forced_challenges,
        })
    }
}

impl BinaryData for FightChallengeMonster {
    const TYPE_ID: i16 = 132;
}
