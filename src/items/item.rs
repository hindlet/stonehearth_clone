use bevy::prelude::*;

use crate::VoxelTextureHandle;

use super::{item_meshes::{get_item_mesh, ItemMeshes}, item_template::{CraftingMaterial, Equipment, ItemTemplate, MainHandWeapon}};











#[derive(Component)]
pub struct ItemInfo {
    pub display_name: String
}

#[derive(Component)]
pub struct ItemEquipmentTag;






pub fn spawn_item_from_template(
    commands: &mut Commands,
    template_handle: Handle<ItemTemplate>,
    item_templates: &Assets<ItemTemplate>,

    item_meshes: &mut ItemMeshes,
    mesh_assets: &mut Assets<Mesh> ,
    material: &VoxelTextureHandle,
    spawn_pos: Vec3,
) {
    if let Some(template) = item_templates.get(&template_handle) {
        match template {
            ItemTemplate::Equipable(equipment) => {
                match equipment {
                    Equipment::MainHandWeapon(MainHandWeapon {info , atk_speed, atk_dmg}) => {
                        commands
                            .spawn(ItemEquipmentTag)
                            .insert(ItemInfo {
                                display_name: info.name.clone()
                            })
                            .insert(PbrBundle {
                                mesh: get_item_mesh(item_meshes, mesh_assets, info),
                                material: material.material_handle.clone_weak(),
                                transform: Transform::from_translation(spawn_pos),
                                ..Default::default()
                            });
                    },
                    _ => ()
                }
            },
            _ => ()
        }
    }
}



// pub fn spawn_item_from_template(
//     commands: &mut Commands,
//     template_handle: &Handle<ItemTemplate>,
//     item_templates: Res<Assets<ItemTemplate>>,

    
// ) {
//     if let Some(template) = item_templates.get(template_handle) {
//         match template {
//             ItemTemplate::InventoryItem(CraftingMaterial{info}) => {
                
//             },
//             ItemTemplate::Equipable(equipment) => {
//                 match equipment {
//                     Equipment::MainHandWeapon(MainHandWeapon{info, ..}) => {
                        
//                     },
//                     Equipment::OffhandWeapon() => {},
//                     Equipment::Shield() => {},
//                     Equipment::TwoHandWeapon() => {},
//                 }
//             }
//         }
//     }
// }