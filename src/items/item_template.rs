use bevy::{prelude::Asset, reflect::TypePath};
use serde::{Serialize, Deserialize};
use super::ItemTrait;





#[derive(Serialize, Deserialize, Asset, TypePath, Clone, Debug)]
pub enum ItemTemplate {
    InventoryItem(CraftingMaterial),
    Equipable(Equipment)
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Hash, Eq)]
pub struct ItemTemplateInfo {
    pub name: String,
    pub voxel_path: String,
}




#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CraftingMaterial {
    pub info: ItemTemplateInfo
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Equipment {
    MainHandWeapon(MainHandWeapon),
    OffhandWeapon(),
    Shield(),
    TwoHandWeapon(),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MainHandWeapon {
    pub info: ItemTemplateInfo,
    pub atk_speed: f32,
    pub atk_dmg: f32,
}