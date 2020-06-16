use crate::BinaryData;
use crate::decode::{Decode, DecodeState};
use std::io;
use std::marker::PhantomData;

#[derive(Debug, Clone, serde::Serialize)]
pub struct HavenWorldBuildingDefinition {
    pub _0: i32,
    pub _1: i16,
    pub _2: i32,
    pub _3: i32,
    pub _4: i8,
    pub _5: i8,
    pub _6: i32,
    pub _7: bool,
    pub _8: Vec<HavenWorldBuildingDefinition_8>,
    pub _9: Vec<HavenWorldBuildingDefinition_9>,
    pub _10: Vec<i32>,
    pub _11: Vec<HavenWorldBuildingDefinition_11>,
}

impl BinaryData for HavenWorldBuildingDefinition {
    fn id(_phantom: PhantomData<Self>) -> i32 {
        102
    }
}

impl Decode for HavenWorldBuildingDefinition {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let _0 = state.decode()?;
        let _1 = state.decode()?;
        let _2 = state.decode()?;
        let _3 = state.decode()?;
        let _4 = state.decode()?;
        let _5 = state.decode()?;
        let _6 = state.decode()?;
        let _7 = state.decode()?;
        let _8 = state.decode()?;
        let _9 = state.decode()?;
        let _10 = state.decode()?;
        let _11 = state.decode()?;
        Ok(HavenWorldBuildingDefinition {
            _0,
            _1,
            _2,
            _3,
            _4,
            _5,
            _6,
            _7,
            _8,
            _9,
            _10,
            _11,
        })
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct HavenWorldBuildingDefinition_11 {
    pub _0: i32,
}

impl Decode for HavenWorldBuildingDefinition_11 {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let _0 = state.decode()?;
        Ok(HavenWorldBuildingDefinition_11 { _0 })
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct HavenWorldBuildingDefinition_9 {
    pub _0: i32,
    pub _1: i32,
}

impl Decode for HavenWorldBuildingDefinition_9 {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let _0 = state.decode()?;
        let _1 = state.decode()?;
        Ok(HavenWorldBuildingDefinition_9 { _0, _1 })
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct HavenWorldBuildingDefinition_8 {
    pub _0: i32,
    pub _1: i32,
    pub _2: i8,
    pub _3: i8,
    pub _4: i8,
}

impl Decode for HavenWorldBuildingDefinition_8 {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let _0 = state.decode()?;
        let _1 = state.decode()?;
        let _2 = state.decode()?;
        let _3 = state.decode()?;
        let _4 = state.decode()?;
        Ok(HavenWorldBuildingDefinition_8 { _0, _1, _2, _3, _4 })
    }
}
