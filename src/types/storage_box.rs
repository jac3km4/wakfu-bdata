use crate::BinaryData;
use crate::decode::{Decode, DecodeState};
use std::io;
use std::marker::PhantomData;

#[derive(Debug, Clone, serde::Serialize)]
pub struct StorageBox {
    pub id: i32,
    pub visual_id: i32,
    pub _2: StorageBox_2,
    pub compartments: Vec<StorageBoxCompartments>,
}

impl BinaryData for StorageBox {
    fn id(_phantom: PhantomData<Self>) -> i32 {
        70
    }
}

impl Decode for StorageBox {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        let visual_id = state.decode()?;
        let _2 = state.decode()?;
        let compartments = state.decode()?;
        Ok(StorageBox {
            id,
            visual_id,
            _2,
            compartments,
        })
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct StorageBoxCompartments {
    pub uid: i32,
    pub box_id: i32,
    pub unlock_item_id: i32,
    pub capacity: i8,
    pub compartment_order: i32,
}

impl Decode for StorageBoxCompartments {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let uid = state.decode()?;
        let box_id = state.decode()?;
        let unlock_item_id = state.decode()?;
        let capacity = state.decode()?;
        let compartment_order = state.decode()?;
        Ok(StorageBoxCompartments {
            uid,
            box_id,
            unlock_item_id,
            capacity,
            compartment_order,
        })
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct StorageBox_2 {
    pub _0: i8,
    pub _1: i32,
}

impl Decode for StorageBox_2 {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let _0 = state.decode()?;
        let _1 = state.decode()?;
        Ok(StorageBox_2 { _0, _1 })
    }
}
