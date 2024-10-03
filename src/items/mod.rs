mod item_template;
mod item;
mod item_meshes;


use std::fmt::Debug;

use bevy::{app::{PostStartup, Startup}, asset::{AssetServer, Handle}, prelude::{Plugin, Res, ResMut}, utils::hashbrown::HashMap};
use bevy_common_assets::ron::RonAssetPlugin;
pub use item_meshes::ItemMeshes;
pub use item_template::ItemTemplate;
pub use item::spawn_item_from_template;


trait ItemTrait: Clone + Debug {}

pub struct ItemPlugin;

impl Plugin for ItemPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .add_plugins(RonAssetPlugin::<ItemTemplate>::new(&["item.ron"]))
            .insert_resource(ItemMeshes::new());
            // .add_systems(Startup, test)
            // .add_systems(PostStartup, draw_test_sword);
    }
}
