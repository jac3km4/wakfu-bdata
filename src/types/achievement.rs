use crate::BinaryData;
use crate::decode::{Decode, DecodeState};
use std::io;
use std::marker::PhantomData;

#[derive(Debug, Clone, serde::Serialize)]
pub struct Achievement {
    pub id: i32,
    pub category_id: i32,
    pub is_visible: bool,
    pub notify_on_pass: bool,
    pub is_active: bool,
    pub criterion: String,
    pub activation_criterion: String,
    pub _7: Vec<Achievement_7>,
    pub _8: Vec<Achievement_8>,
    pub duration: i32,
    pub cooldown: i32,
    pub shareable: bool,
    pub repeatable: bool,
    pub needs_user_accept: bool,
    pub recommanded_level: i32,
    pub recommanded_players: i32,
    pub followable: bool,
    pub display_on_activation_delay: i32,
    pub period_start_time: i64,
    pub period: i64,
    pub auto_compass: bool,
    pub gfx_id: i32,
    pub is_mercenary: bool,
    pub mercenary_item_id: i32,
    pub mercenary_rank: i8,
    pub order: i32,
    pub _26: String,
    pub _27: i32,
}

impl BinaryData for Achievement {
    fn id(_phantom: PhantomData<Self>) -> i32 {
        1
    }
}

impl Decode for Achievement {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        let category_id = state.decode()?;
        let is_visible = state.decode()?;
        let notify_on_pass = state.decode()?;
        let is_active = state.decode()?;
        let criterion = state.decode()?;
        let activation_criterion = state.decode()?;
        let _7 = state.decode()?;
        let _8 = state.decode()?;
        let duration = state.decode()?;
        let cooldown = state.decode()?;
        let shareable = state.decode()?;
        let repeatable = state.decode()?;
        let needs_user_accept = state.decode()?;
        let recommanded_level = state.decode()?;
        let recommanded_players = state.decode()?;
        let followable = state.decode()?;
        let display_on_activation_delay = state.decode()?;
        let period_start_time = state.decode()?;
        let period = state.decode()?;
        let auto_compass = state.decode()?;
        let gfx_id = state.decode()?;
        let is_mercenary = state.decode()?;
        let mercenary_item_id = state.decode()?;
        let mercenary_rank = state.decode()?;
        let order = state.decode()?;
        let _26 = state.decode()?;
        let _27 = state.decode()?;
        Ok(Achievement {
            id,
            category_id,
            is_visible,
            notify_on_pass,
            is_active,
            criterion,
            activation_criterion,
            _7,
            _8,
            duration,
            cooldown,
            shareable,
            repeatable,
            needs_user_accept,
            recommanded_level,
            recommanded_players,
            followable,
            display_on_activation_delay,
            period_start_time,
            period,
            auto_compass,
            gfx_id,
            is_mercenary,
            mercenary_item_id,
            mercenary_rank,
            order,
            _26,
            _27,
        })
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct Achievement_8 {
    pub _0: i32,
    pub _1: i32,
    pub _2: bool,
    pub _3: Vec<i32>,
}

impl Decode for Achievement_8 {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let _0 = state.decode()?;
        let _1 = state.decode()?;
        let _2 = state.decode()?;
        let _3 = state.decode()?;
        Ok(Achievement_8 { _0, _1, _2, _3 })
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct Achievement_7_5 {
    pub _0: i32,
    pub _1: String,
    pub _2: Vec<i32>,
}

impl Decode for Achievement_7_5 {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let _0 = state.decode()?;
        let _1 = state.decode()?;
        let _2 = state.decode()?;
        Ok(Achievement_7_5 { _0, _1, _2 })
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct Achievement_7_3 {
    pub _0: i16,
    pub _1: i16,
    pub _2: i16,
    pub _3: i64,
}

impl Decode for Achievement_7_3 {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let _0 = state.decode()?;
        let _1 = state.decode()?;
        let _2 = state.decode()?;
        let _3 = state.decode()?;
        Ok(Achievement_7_3 { _0, _1, _2, _3 })
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct Achievement_7 {
    pub _0: i32,
    pub _1: bool,
    pub _2: String,
    pub _3: Vec<Achievement_7_3>,
    pub _4: String,
    pub _5: Vec<Achievement_7_5>,
}

impl Decode for Achievement_7 {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let _0 = state.decode()?;
        let _1 = state.decode()?;
        let _2 = state.decode()?;
        let _3 = state.decode()?;
        let _4 = state.decode()?;
        let _5 = state.decode()?;
        Ok(Achievement_7 { _0, _1, _2, _3, _4, _5 })
    }
}
