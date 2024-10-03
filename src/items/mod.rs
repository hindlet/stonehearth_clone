mod item_template;
mod item;


use std::fmt::Debug;

use bevy::{app::{PostStartup, Startup}, asset::{AssetServer, Handle}, prelude::{Plugin, Res, ResMut}};
use bevy_common_assets::ron::RonAssetPlugin;
use item_template::ItemTemplate;


trait ItemTrait: Clone + Debug {}

pub struct ItemPlugin;

impl Plugin for ItemPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .add_plugins(RonAssetPlugin::<ItemTemplate>::new(&["item.ron"]));
            // .add_systems(Startup, test)
            // .add_systems(PostStartup, draw_test_sword);
    }
}
