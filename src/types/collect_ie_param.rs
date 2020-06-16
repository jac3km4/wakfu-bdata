use crate::BinaryData;
use crate::decode::{Decode, DecodeState};
use std::io;
use std::marker::PhantomData;

#[derive(Debug, Clone, serde::Serialize)]
pub struct CollectIeParam {
    pub id: i32,
    pub visual_id: i32,
    pub capacity: i16,
    pub locked: bool,
    pub cash_qty: i32,
    pub items: Vec<CollectIeParamItems>,
    pub actions: Vec<CollectIeParamActions>,
}

impl BinaryData for CollectIeParam {
    fn id(_phantom: PhantomData<Self>) -> i32 {
        22
    }
}

impl Decode for CollectIeParam {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        let visual_id = state.decode()?;
        let capacity = state.decode()?;
        let locked = state.decode()?;
        let cash_qty = state.decode()?;
        let items = state.decode()?;
        let actions = state.decode()?;
        Ok(CollectIeParam {
            id,
            visual_id,
            capacity,
            locked,
            cash_qty,
            items,
            actions,
        })
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct CollectIeParamActions {
    pub action_id: i32,
    pub action_type: i32,
    pub params: Vec<String>,
    pub criteria: String,
}

impl Decode for CollectIeParamActions {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let action_id = state.decode()?;
        let action_type = state.decode()?;
        let params = state.decode()?;
        let criteria = state.decode()?;
        Ok(CollectIeParamActions {
            action_id,
            action_type,
            params,
            criteria,
        })
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct CollectIeParamItems {
    pub uid: i32,
    pub item_id: i32,
    pub qty: i32,
}

impl Decode for CollectIeParamItems {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let uid = state.decode()?;
        let item_id = state.decode()?;
        let qty = state.decode()?;
        Ok(CollectIeParamItems { uid, item_id, qty })
    }
}
