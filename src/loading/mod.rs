/// Module for Loading all of the game data
/// 
/// Divided up into 4 parts
/// 1: AssetLoading
///     - Loads the basic assets needed for the game
/// 2: ItemLoading
///     - Loads all the item templates
/// 3: VoxelLoading
///     - Loads all voxel information
/// 4: EntityLoading
///     - Loads all entity information
use bevy::{asset::{LoadState, LoadedFolder}, prelude::*};

use crate::AppState;


#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
enum LoadingState {
    #[default]
    BasicAssetLoading,
    ItemLoading,
    UISpriteLoading,
    VoxelLoading,
    EntityLoading,
    LoadingComplete,
}


pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_state::<LoadingState>()
            
            .add_systems(OnEnter(AppState::Loading), load_basic_assets)
            .add_systems(Update, check_basic_assets_loaded.run_if(in_state(LoadingState::BasicAssetLoading)).run_if(in_state(AppState::Loading)))

            .add_systems(OnEnter(LoadingState::ItemLoading), load_item_templates)
            .add_systems(Update, check_item_templates_loaded.run_if(in_state(LoadingState::ItemLoading)))

            .add_systems(OnEnter(LoadingState::UISpriteLoading), load_ui_spritres)
            .add_systems(Update, check_ui_sprites_loaded.run_if(in_state(LoadingState::UISpriteLoading)))
            

            .add_systems(OnEnter(LoadingState::LoadingComplete), complete_loading)
            
            ;
    }
}




fn complete_loading(
    mut next_state: ResMut<NextState<AppState>>
) {
    println!("Loading Complete!");
    next_state.set(AppState::MainMenu);
}


/// Voxel Loading

#[derive(Resource)]
pub struct VoxelTextureHandle {
    pub material_handle: Handle<StandardMaterial>,
    pub _image_handle: Handle<Image>
}



fn load_basic_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    println!("Loading Basic Assets");
    let image_handle: Handle<Image> = asset_server.load("palette.png");
    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(image_handle.clone()),
        ..Default::default()
    });

    commands.insert_resource(VoxelTextureHandle {
        _image_handle: image_handle,
        material_handle
    })
}


fn check_basic_assets_loaded(
    voxel_texture_handles: Res<VoxelTextureHandle>,
    asset_server: Res<AssetServer>,
    mut next_state: ResMut<NextState<LoadingState>>
) {
    if let Some(state) = asset_server.get_load_state(&voxel_texture_handles._image_handle) {
        if state == LoadState::Loaded {
            println!("Basic Assets Loaded");
            next_state.set(LoadingState::ItemLoading);
        }
    }
}





/// Item Template Loading

#[derive(Resource)]
pub struct ItemTemplateHandles(Handle<LoadedFolder>);


fn load_item_templates(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    println!("Loading Item Templates");
    commands.insert_resource(ItemTemplateHandles(asset_server.load_folder("item_templates")));
}

fn check_item_templates_loaded(
    item_template_handles: Res<ItemTemplateHandles>,
    asset_server: Res<AssetServer>,
    mut next_state: ResMut<NextState<LoadingState>>
) {

    if let Some(state) = asset_server.get_load_state(&item_template_handles.0) {
        if state == LoadState::Loaded {
            println!("Item Templates Loaded");
            next_state.set(LoadingState::UISpriteLoading);
        }
    }
}


/// UI Loading

#[derive(Resource)]
pub struct UISpriteHandles(Handle<LoadedFolder>);

fn load_ui_spritres(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    println!("Loading UI Sprites");
    commands.insert_resource(UISpriteHandles(asset_server.load_folder("ui_sprites")));
}


fn check_ui_sprites_loaded(
    item_template_handles: Res<UISpriteHandles>,
    asset_server: Res<AssetServer>,
    mut next_state: ResMut<NextState<LoadingState>>
) {

    if let Some(state) = asset_server.get_load_state(&item_template_handles.0) {
        if state == LoadState::Loaded {
            println!("Loaded UI Sprites");
            next_state.set(LoadingState::LoadingComplete);
        }
    }
}