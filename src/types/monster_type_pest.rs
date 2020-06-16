use crate::BinaryData;
use crate::decode::{Decode, DecodeState};
use std::io;
use std::marker::PhantomData;

#[derive(Debug, Clone, serde::Serialize)]
pub struct MonsterTypePest {
    pub id: i32,
    pub family_id: i32,
    pub pest_monster_id: i32,
}

impl BinaryData for MonsterTypePest {
    fn id(_phantom: PhantomData<Self>) -> i32 {
        48
    }
}

impl Decode for MonsterTypePest {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        let family_id = state.decode()?;
        let pest_monster_id = state.decode()?;
        Ok(MonsterTypePest {
            id,
            family_id,
            pest_monster_id,
        })
    }
}
