use crate::BinaryData;
use crate::decode::{Decode, DecodeState};
use std::io;
use std::marker::PhantomData;

#[derive(Debug, Clone, serde::Serialize)]
pub struct EquipableDummyIeParam {
    pub id: i32,
    pub anim_name: String,
    pub sex: i8,
}

impl BinaryData for EquipableDummyIeParam {
    fn id(_phantom: PhantomData<Self>) -> i32 {
        106
    }
}

impl Decode for EquipableDummyIeParam {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        let anim_name = state.decode()?;
        let sex = state.decode()?;
        Ok(EquipableDummyIeParam { id, anim_name, sex })
    }
}
