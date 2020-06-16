use crate::BinaryData;
use crate::decode::{Decode, DecodeState};
use std::io;
use std::marker::PhantomData;

#[derive(Debug, Clone, serde::Serialize)]
pub struct NationLaw {
    pub id: i32,
    pub law_constant_id: i32,
    pub params: Vec<String>,
    pub base_points_modification: i32,
    pub percent_points_modification: i32,
    pub law_point_cost: i32,
    pub law_locked: bool,
    pub applicable_to_citizen: bool,
    pub applicable_to_allied_foreigner: bool,
    pub applicable_to_neutral_foreigner: bool,
    pub restricted_nations: Vec<i32>,
}

impl BinaryData for NationLaw {
    fn id(_phantom: PhantomData<Self>) -> i32 {
        52
    }
}

impl Decode for NationLaw {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        let law_constant_id = state.decode()?;
        let params = state.decode()?;
        let base_points_modification = state.decode()?;
        let percent_points_modification = state.decode()?;
        let law_point_cost = state.decode()?;
        let law_locked = state.decode()?;
        let applicable_to_citizen = state.decode()?;
        let applicable_to_allied_foreigner = state.decode()?;
        let applicable_to_neutral_foreigner = state.decode()?;
        let restricted_nations = state.decode()?;
        Ok(NationLaw {
            id,
            law_constant_id,
            params,
            base_points_modification,
            percent_points_modification,
            law_point_cost,
            law_locked,
            applicable_to_citizen,
            applicable_to_allied_foreigner,
            applicable_to_neutral_foreigner,
            restricted_nations,
        })
    }
}
