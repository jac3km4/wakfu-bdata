use crate::BinaryData;
use crate::decode::{Decode, DecodeState};
use std::io;
use std::marker::PhantomData;

#[derive(Debug, Clone, serde::Serialize)]
pub struct KrosmozGameCollectionIeParam {
    pub id: i32,
    pub game_id: i8,
}

impl BinaryData for KrosmozGameCollectionIeParam {
    fn id(_phantom: PhantomData<Self>) -> i32 {
        109
    }
}

impl Decode for KrosmozGameCollectionIeParam {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        let game_id = state.decode()?;
        Ok(KrosmozGameCollectionIeParam { id, game_id })
    }
}
