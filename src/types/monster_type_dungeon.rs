use crate::BinaryData;
use crate::decode::{Decode, DecodeState};
use std::io;
use std::marker::PhantomData;

#[derive(Debug, Clone, serde::Serialize)]
pub struct MonsterTypeDungeon {
    pub id: i32,
    pub family_id: i32,
    pub dungeon_id: i32,
    pub level: i16,
}

impl BinaryData for MonsterTypeDungeon {
    fn id(_phantom: PhantomData<Self>) -> i32 {
        137
    }
}

impl Decode for MonsterTypeDungeon {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        let family_id = state.decode()?;
        let dungeon_id = state.decode()?;
        let level = state.decode()?;
        Ok(MonsterTypeDungeon {
            id,
            family_id,
            dungeon_id,
            level,
        })
    }
}
