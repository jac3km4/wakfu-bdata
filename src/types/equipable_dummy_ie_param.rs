use std::io;

use serde::Serialize;

use crate::data::BinaryData;
use crate::decode::{Decode, DecodeState};

#[derive(Debug, Clone, Serialize)]
pub struct EquipableDummyIeParam {
    pub id: i32,
    pub anim_name: String,
    pub sex: i8,
}

impl Decode for EquipableDummyIeParam {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        let anim_name = state.decode()?;
        let sex = state.decode()?;
        Ok(Self { id, anim_name, sex })
    }
}

impl BinaryData for EquipableDummyIeParam {
    const TYPE_ID: i16 = 106;
}
