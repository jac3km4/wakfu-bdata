use std::io;

use serde::Serialize;

use crate::data::BinaryData;
use crate::decode::{Decode, DecodeState};

#[derive(Debug, Clone, Serialize)]
pub struct ProtectorEcosystemProtection {
    pub protector_id: i32,
    pub fauna_protection: Vec<ProtectorEcosystemProtectionFaunaProtection>,
    pub flora_protection: Vec<ProtectorEcosystemProtectionFloraProtection>,
}

impl Decode for ProtectorEcosystemProtection {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let protector_id = state.decode()?;
        let fauna_protection = state.decode()?;
        let flora_protection = state.decode()?;
        Ok(Self {
            protector_id,
            fauna_protection,
            flora_protection,
        })
    }
}

impl BinaryData for ProtectorEcosystemProtection {
    const TYPE_ID: i16 = 57;
}

#[derive(Debug, Clone, Serialize)]
pub struct ProtectorEcosystemProtectionFaunaProtection {
    pub monster_family_id: i32,
    pub protection_cost: i32,
    pub reintroduction_cost: i32,
    pub reintroduction_item_id: i32,
    pub reintroduction_item_qty: i16,
}

impl Decode for ProtectorEcosystemProtectionFaunaProtection {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let monster_family_id = state.decode()?;
        let protection_cost = state.decode()?;
        let reintroduction_cost = state.decode()?;
        let reintroduction_item_id = state.decode()?;
        let reintroduction_item_qty = state.decode()?;
        Ok(Self {
            monster_family_id,
            protection_cost,
            reintroduction_cost,
            reintroduction_item_id,
            reintroduction_item_qty,
        })
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct ProtectorEcosystemProtectionFloraProtection {
    pub resource_family_id: i32,
    pub protection_cost: i32,
    pub reintroduction_cost: i32,
    pub reintroduction_item_id: i32,
    pub reintroduction_item_qty: i16,
}

impl Decode for ProtectorEcosystemProtectionFloraProtection {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let resource_family_id = state.decode()?;
        let protection_cost = state.decode()?;
        let reintroduction_cost = state.decode()?;
        let reintroduction_item_id = state.decode()?;
        let reintroduction_item_qty = state.decode()?;
        Ok(Self {
            resource_family_id,
            protection_cost,
            reintroduction_cost,
            reintroduction_item_id,
            reintroduction_item_qty,
        })
    }
}
