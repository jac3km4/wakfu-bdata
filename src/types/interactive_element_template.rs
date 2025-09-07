use std::io;

use serde::Serialize;

use crate::data::BinaryData;
use crate::decode::{Decode, DecodeState};

#[derive(Debug, Clone, Serialize)]
pub struct InteractiveElementTemplate {
    pub id: i32,
    pub model_type: i16,
    pub world_id: i16,
    pub x: i32,
    pub y: i32,
    pub z: i16,
    pub initial_state: i16,
    pub initially_visible: bool,
    pub initially_usable: bool,
    pub blocking_movement: bool,
    pub blocking_los: bool,
    pub direction: i8,
    pub activation_pattern: i16,
    pub parameter: String,
    pub template_id: i32,
    pub properties: Vec<i32>,
    pub positions_trigger: Vec<InteractiveElementTemplatePositionsTrigger>,
    pub actions: std::collections::HashMap<i16, i32>,
    pub views: Vec<i32>,
}

impl Decode for InteractiveElementTemplate {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        let model_type = state.decode()?;
        let world_id = state.decode()?;
        let x = state.decode()?;
        let y = state.decode()?;
        let z = state.decode()?;
        let initial_state = state.decode()?;
        let initially_visible = state.decode()?;
        let initially_usable = state.decode()?;
        let blocking_movement = state.decode()?;
        let blocking_los = state.decode()?;
        let direction = state.decode()?;
        let activation_pattern = state.decode()?;
        let parameter = state.decode()?;
        let template_id = state.decode()?;
        let properties = state.decode()?;
        let positions_trigger = state.decode()?;
        let actions = state.decode()?;
        let views = state.decode()?;
        Ok(Self {
            id,
            model_type,
            world_id,
            x,
            y,
            z,
            initial_state,
            initially_visible,
            initially_usable,
            blocking_movement,
            blocking_los,
            direction,
            activation_pattern,
            parameter,
            template_id,
            properties,
            positions_trigger,
            actions,
            views,
        })
    }
}

impl BinaryData for InteractiveElementTemplate {
    const TYPE_ID: i16 = 128;
}

#[derive(Debug, Clone, Serialize)]
pub struct InteractiveElementTemplatePositionsTrigger {
    pub x: i32,
    pub y: i32,
    pub z: i16,
}

impl Decode for InteractiveElementTemplatePositionsTrigger {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let x = state.decode()?;
        let y = state.decode()?;
        let z = state.decode()?;
        Ok(Self { x, y, z })
    }
}
