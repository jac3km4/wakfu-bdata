use crate::BinaryData;
use crate::decode::{Decode, DecodeState};
use std::io;
use std::marker::PhantomData;

#[derive(Debug, Clone, serde::Serialize)]
pub struct ClientEvent {
    pub id: i32,
    pub kind: i32,
    pub drop_rate: i16,
    pub max_count: i16,
    pub criterion: String,
    pub filters: Vec<String>,
    pub active_on_start: bool,
    pub action_groups: Vec<ClientEventActionGroups>,
}

impl BinaryData for ClientEvent {
    fn id(_phantom: PhantomData<Self>) -> i32 {
        18
    }
}

impl Decode for ClientEvent {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        let kind = state.decode()?;
        let drop_rate = state.decode()?;
        let max_count = state.decode()?;
        let criterion = state.decode()?;
        let filters = state.decode()?;
        let active_on_start = state.decode()?;
        let action_groups = state.decode()?;
        Ok(ClientEvent {
            id,
            kind,
            drop_rate,
            max_count,
            criterion,
            filters,
            active_on_start,
            action_groups,
        })
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct ClientEventActionGroupsActions {
    pub id: i32,
    pub kind: i32,
    pub params: Vec<String>,
}

impl Decode for ClientEventActionGroupsActions {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        let kind = state.decode()?;
        let params = state.decode()?;
        Ok(ClientEventActionGroupsActions { id, kind, params })
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct ClientEventActionGroups {
    pub id: i32,
    pub drop_rate: i16,
    pub criterion: String,
    pub actions: Vec<ClientEventActionGroupsActions>,
}

impl Decode for ClientEventActionGroups {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        let drop_rate = state.decode()?;
        let criterion = state.decode()?;
        let actions = state.decode()?;
        Ok(ClientEventActionGroups {
            id,
            drop_rate,
            criterion,
            actions,
        })
    }
}
