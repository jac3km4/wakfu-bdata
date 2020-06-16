use crate::BinaryData;
use crate::decode::{Decode, DecodeState};
use std::io;
use std::marker::PhantomData;

#[derive(Debug, Clone, serde::Serialize)]
pub struct AvatarBreed {
    pub id: i32,
    pub name: String,
    pub background_aps: i32,
    pub base_hp: i32,
    pub base_ap: i32,
    pub base_mp: i32,
    pub base_wp: i32,
    pub base_init: i32,
    pub base_ferocity: i32,
    pub base_fumble: i32,
    pub base_wisdom: i32,
    pub base_tackle: i32,
    pub base_dodge: i32,
    pub base_prospection: i32,
    pub timer_count_before_death: i32,
    pub prefered_area: i32,
    pub spell_elements: Vec<i8>,
    pub charac_ratios: Vec<f32>,
    pub _18: Vec<i16>,
}

impl BinaryData for AvatarBreed {
    fn id(_phantom: PhantomData<Self>) -> i32 {
        86
    }
}

impl Decode for AvatarBreed {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        let name = state.decode()?;
        let background_aps = state.decode()?;
        let base_hp = state.decode()?;
        let base_ap = state.decode()?;
        let base_mp = state.decode()?;
        let base_wp = state.decode()?;
        let base_init = state.decode()?;
        let base_ferocity = state.decode()?;
        let base_fumble = state.decode()?;
        let base_wisdom = state.decode()?;
        let base_tackle = state.decode()?;
        let base_dodge = state.decode()?;
        let base_prospection = state.decode()?;
        let timer_count_before_death = state.decode()?;
        let prefered_area = state.decode()?;
        let spell_elements = state.decode()?;
        let charac_ratios = state.decode()?;
        let _18 = state.decode()?;
        Ok(AvatarBreed {
            id,
            name,
            background_aps,
            base_hp,
            base_ap,
            base_mp,
            base_wp,
            base_init,
            base_ferocity,
            base_fumble,
            base_wisdom,
            base_tackle,
            base_dodge,
            base_prospection,
            timer_count_before_death,
            prefered_area,
            spell_elements,
            charac_ratios,
            _18,
        })
    }
}
