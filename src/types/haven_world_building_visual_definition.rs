use std::io;

use serde::Serialize;

use crate::data::BinaryData;
use crate::decode::{Decode, DecodeState};

#[derive(Debug, Clone, Serialize)]
pub struct HavenWorldBuildingVisualDefinition {
    pub id: i32,
    pub building_id: i32,
    pub elements: Vec<HavenWorldBuildingVisualDefinitionElements>,
}

impl Decode for HavenWorldBuildingVisualDefinition {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        let building_id = state.decode()?;
        let elements = state.decode()?;
        Ok(Self {
            id,
            building_id,
            elements,
        })
    }
}

impl BinaryData for HavenWorldBuildingVisualDefinition {
    const TYPE_ID: i16 = 107;
}

#[derive(Debug, Clone, Serialize)]
pub struct HavenWorldBuildingVisualDefinitionElements {
    pub uid: i32,
    pub gfx_id: i32,
    pub has_guild_color: bool,
    pub occluder: bool,
    pub height: i8,
    pub anim_name: String,
    pub direction: i8,
    pub x: i8,
    pub y: i8,
    pub z: i8,
}

impl Decode for HavenWorldBuildingVisualDefinitionElements {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let uid = state.decode()?;
        let gfx_id = state.decode()?;
        let has_guild_color = state.decode()?;
        let occluder = state.decode()?;
        let height = state.decode()?;
        let anim_name = state.decode()?;
        let direction = state.decode()?;
        let x = state.decode()?;
        let y = state.decode()?;
        let z = state.decode()?;
        Ok(Self {
            uid,
            gfx_id,
            has_guild_color,
            occluder,
            height,
            anim_name,
            direction,
            x,
            y,
            z,
        })
    }
}
