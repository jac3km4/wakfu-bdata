use crate::BinaryData;
use crate::decode::{Decode, DecodeState};
use std::io;
use std::marker::PhantomData;

#[derive(Debug, Clone, serde::Serialize)]
pub struct EffectGroup {
    pub id: i32,
    pub effect_ids: Vec<i32>,
}

impl BinaryData for EffectGroup {
    fn id(_phantom: PhantomData<Self>) -> i32 {
        29
    }
}

impl Decode for EffectGroup {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        let effect_ids = state.decode()?;
        Ok(EffectGroup { id, effect_ids })
    }
}
