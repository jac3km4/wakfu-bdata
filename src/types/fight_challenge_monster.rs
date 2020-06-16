use crate::BinaryData;
use crate::decode::{Decode, DecodeState};
use std::io;
use std::marker::PhantomData;

#[derive(Debug, Clone, serde::Serialize)]
pub struct FightChallengeMonster {
    pub id: i32,
    pub random_rolls: i16,
    pub forced_rolls: i16,
    pub forced_challenges: Vec<i32>,
}

impl BinaryData for FightChallengeMonster {
    fn id(_phantom: PhantomData<Self>) -> i32 {
        132
    }
}

impl Decode for FightChallengeMonster {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        let random_rolls = state.decode()?;
        let forced_rolls = state.decode()?;
        let forced_challenges = state.decode()?;
        Ok(FightChallengeMonster {
            id,
            random_rolls,
            forced_rolls,
            forced_challenges,
        })
    }
}
