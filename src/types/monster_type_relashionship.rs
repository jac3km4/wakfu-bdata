use crate::BinaryData;
use crate::decode::{Decode, DecodeState};
use std::io;
use std::marker::PhantomData;

#[derive(Debug, Clone, serde::Serialize)]
pub struct MonsterTypeRelashionship {
    pub id: i32,
    pub family_from: i32,
    pub family_to: i32,
}

impl BinaryData for MonsterTypeRelashionship {
    fn id(_phantom: PhantomData<Self>) -> i32 {
        49
    }
}

impl Decode for MonsterTypeRelashionship {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        let family_from = state.decode()?;
        let family_to = state.decode()?;
        Ok(MonsterTypeRelashionship {
            id,
            family_from,
            family_to,
        })
    }
}
