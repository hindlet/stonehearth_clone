use bevy::prelude::*;

use super::item_template::{CraftingMaterial, Equipment, ItemTemplate, MainHandWeapon};









#[derive(Component)]
pub struct ItemInfo {
    pub display_name: String
}




#[derive(Component)]
pub struct ItemEquipment {
    pub mesh: Handle<Mesh>,
}




pub fn spawn_item_from_template(
    commands: &mut Commands,
    template_handle: Handle<ItemTemplate>,
    item_templates: Res<Assets<ItemTemplate>>
) {

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