use crate::BinaryData;
use crate::decode::{Decode, DecodeState};
use std::io;
use std::marker::PhantomData;

#[derive(Debug, Clone, serde::Serialize)]
pub struct KrosmozGameBoardIeParam {
    pub id: i32,
    pub game_id: i8,
}

impl BinaryData for KrosmozGameBoardIeParam {
    fn id(_phantom: PhantomData<Self>) -> i32 {
        108
    }
}

impl Decode for KrosmozGameBoardIeParam {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        let game_id = state.decode()?;
        Ok(KrosmozGameBoardIeParam { id, game_id })
    }
}
