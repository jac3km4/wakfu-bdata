use crate::BinaryData;
use crate::decode::{Decode, DecodeState};
use std::io;
use std::marker::PhantomData;

#[derive(Debug, Clone, serde::Serialize)]
pub struct AvatarBreedColors {
    pub id: i32,
    pub _1: Vec<AvatarBreedColors_1>,
    pub _2: Vec<AvatarBreedColors_2>,
    pub _3: AvatarBreedColors_3,
    pub _4: AvatarBreedColors_4,
    pub _5: std::collections::HashMap<i16, i8>,
    pub _6: std::collections::HashMap<i16, i8>,
}

impl BinaryData for AvatarBreedColors {
    fn id(_phantom: PhantomData<Self>) -> i32 {
        116
    }
}

impl Decode for AvatarBreedColors {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        let _1 = state.decode()?;
        let _2 = state.decode()?;
        let _3 = state.decode()?;
        let _4 = state.decode()?;
        let _5 = state.decode()?;
        let _6 = state.decode()?;
        Ok(AvatarBreedColors {
            id,
            _1,
            _2,
            _3,
            _4,
            _5,
            _6,
        })
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct AvatarBreedColors_4 {
    pub _0: i8,
    pub _1: i8,
}

impl Decode for AvatarBreedColors_4 {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let _0 = state.decode()?;
        let _1 = state.decode()?;
        Ok(AvatarBreedColors_4 { _0, _1 })
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct AvatarBreedColors_3 {
    pub _0: i8,
    pub _1: i8,
}

impl Decode for AvatarBreedColors_3 {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let _0 = state.decode()?;
        let _1 = state.decode()?;
        Ok(AvatarBreedColors_3 { _0, _1 })
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct AvatarBreedColors_2 {
    pub _0: i32,
    pub _1: i8,
    pub _2: f32,
    pub _3: f32,
    pub _4: f32,
    pub _5: f32,
    pub _6: i16,
}

impl Decode for AvatarBreedColors_2 {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let _0 = state.decode()?;
        let _1 = state.decode()?;
        let _2 = state.decode()?;
        let _3 = state.decode()?;
        let _4 = state.decode()?;
        let _5 = state.decode()?;
        let _6 = state.decode()?;
        Ok(AvatarBreedColors_2 {
            _0,
            _1,
            _2,
            _3,
            _4,
            _5,
            _6,
        })
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct AvatarBreedColors_1 {
    pub _0: i32,
    pub _1: i8,
    pub _2: f32,
    pub _3: f32,
    pub _4: f32,
    pub _5: f32,
    pub _6: i16,
}

impl Decode for AvatarBreedColors_1 {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let _0 = state.decode()?;
        let _1 = state.decode()?;
        let _2 = state.decode()?;
        let _3 = state.decode()?;
        let _4 = state.decode()?;
        let _5 = state.decode()?;
        let _6 = state.decode()?;
        Ok(AvatarBreedColors_1 {
            _0,
            _1,
            _2,
            _3,
            _4,
            _5,
            _6,
        })
    }
}
