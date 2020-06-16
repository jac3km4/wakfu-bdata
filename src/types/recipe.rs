use crate::BinaryData;
use crate::decode::{Decode, DecodeState};
use std::io;
use std::marker::PhantomData;

#[derive(Debug, Clone, serde::Serialize)]
pub struct Recipe {
    pub id: i32,
    pub category_id: i32,
    pub duration: i64,
    pub criteria: String,
    pub visibility_criteria: String,
    pub _5: bool,
    pub level: i32,
    pub properties: Vec<i32>,
    pub xp_ratio: i32,
    pub machines_using_recipe: Vec<i32>,
    pub success_rate: i32,
    pub contract_enabled: bool,
    pub needed_kamas: i64,
    pub xp: i64,
    pub _14: i32,
    pub ingredients: Vec<RecipeIngredients>,
    pub products: Vec<RecipeProducts>,
    pub materials: Vec<RecipeMaterials>,
}

impl BinaryData for Recipe {
    fn id(_phantom: PhantomData<Self>) -> i32 {
        58
    }
}

impl Decode for Recipe {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        let category_id = state.decode()?;
        let duration = state.decode()?;
        let criteria = state.decode()?;
        let visibility_criteria = state.decode()?;
        let _5 = state.decode()?;
        let level = state.decode()?;
        let properties = state.decode()?;
        let xp_ratio = state.decode()?;
        let machines_using_recipe = state.decode()?;
        let success_rate = state.decode()?;
        let contract_enabled = state.decode()?;
        let needed_kamas = state.decode()?;
        let xp = state.decode()?;
        let _14 = state.decode()?;
        let ingredients = state.decode()?;
        let products = state.decode()?;
        let materials = state.decode()?;
        Ok(Recipe {
            id,
            category_id,
            duration,
            criteria,
            visibility_criteria,
            _5,
            level,
            properties,
            xp_ratio,
            machines_using_recipe,
            success_rate,
            contract_enabled,
            needed_kamas,
            xp,
            _14,
            ingredients,
            products,
            materials,
        })
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct RecipeMaterials {
    pub min_level: i16,
    pub min_rarity: i16,
    pub optionnal: bool,
    pub material_types: Vec<i32>,
}

impl Decode for RecipeMaterials {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let min_level = state.decode()?;
        let min_rarity = state.decode()?;
        let optionnal = state.decode()?;
        let material_types = state.decode()?;
        Ok(RecipeMaterials {
            min_level,
            min_rarity,
            optionnal,
            material_types,
        })
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct RecipeProducts {
    pub item_id: i32,
    pub quantity: i16,
}

impl Decode for RecipeProducts {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let item_id = state.decode()?;
        let quantity = state.decode()?;
        Ok(RecipeProducts { item_id, quantity })
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct RecipeIngredients {
    pub item_id: i32,
    pub quantity: i16,
    pub _3: i16,
}

impl Decode for RecipeIngredients {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let item_id = state.decode()?;
        let quantity = state.decode()?;
        let _3 = state.decode()?;
        Ok(RecipeIngredients { item_id, quantity, _3 })
    }
}
