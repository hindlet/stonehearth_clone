use std::{fs::File, io::Write};

use bevy::{color::palettes::css::WHITE, pbr::wireframe::{WireframeConfig, WireframePlugin}, prelude::*, render::{settings::{RenderCreation, WgpuFeatures, WgpuSettings}, RenderPlugin}};
mod building;
mod voxels;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use loading::LoadingPlugin;
use maths::{OctaveNoiseGen, SelectiveNoiseGen};
use terrain::spawn_world_chunks;
use ui::UIPlugin;
// use smooth_bevy_cameras::{controllers::fps::{FpsCameraBundle, FpsCameraController, FpsCameraPlugin}, LookTransformPlugin};
use voxels::*;
mod terrain;
mod items;
mod maths;
mod loading;
mod ui;
// mod palette_gen;




const CHUNK_SIDE_LENGTH: u32 = 32;



#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum AppState {
    #[default]
    Setup,

    Loading,

    MainMenu,

    GameLoading,
    Running,
    Paused
}



fn main() {
    // palette_gen::gen_palette();
    App::new()
        .add_plugins(DefaultPlugins.set(RenderPlugin {
            render_creation: RenderCreation::Automatic(WgpuSettings {
                // WARN this is a native only feature. It will not work with webgl or webgpu
                features: WgpuFeatures::POLYGON_MODE_LINE,
                ..default()
            }),
            ..default()
        }))

        .init_state::<AppState>()

        .insert_resource(WireframeConfig {
            global: false,
            default_color: WHITE.into(),
        })

        .add_plugins((
            building::BuildingPlugin, 
            WireframePlugin, 
            voxels::VoxelPlugin,
            LoadingPlugin,
            items::ItemPlugin,
            UIPlugin
        ))

        // .add_plugins((FpsCameraPlugin::default(), LookTransformPlugin))
        .add_plugins(PanOrbitCameraPlugin)
        .add_systems(Startup, spawn_scene_basics)
        .add_systems(Update, toggle_wireframe)
        .add_systems(Startup, temp_change_state)
        .add_systems(OnEnter(AppState::GameLoading), load_the_game_this_is_not_permenant)


        .insert_resource(SelectiveNoiseGen::new(
            345678907654,

            4,
            3.0,
            0.5,
            200.0,

            4,
            2.0,
            0.8,
            100.0,

            100.0,
            20.0
        ))
        .add_systems(OnEnter(AppState::Running), world_gen_test)
        // .add_systems(Update, draw_axis)
        .run();
}


fn temp_change_state(
    mut next_state: ResMut<NextState<AppState>>
) {
    next_state.set(AppState::Loading);
}

fn load_the_game_this_is_not_permenant(
    mut next_state: ResMut<NextState<AppState>>
) {
    next_state.set(AppState::Running);
}



fn world_gen_test(
    mut commands: Commands,
    noise_gen: Res<SelectiveNoiseGen>,
    voxel_texture_handle: Res<VoxelTextureHandle>,
    mut meshes: ResMut<Assets<Mesh>>
) {
    spawn_world_chunks(&mut commands, &noise_gen, voxel_texture_handle.material_handle.clone_weak(), &mut meshes, 10, 1, CHUNK_SIDE_LENGTH);
    println!("Spawned Test Chunks");
}


fn toggle_wireframe(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut config: ResMut<WireframeConfig>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        config.global ^= true;
    }
}

fn spawn_scene_basics(
    mut commands: Commands,
    mut ambient_light: ResMut<AmbientLight>
) {
    ambient_light.brightness = 0.0;

    commands.spawn(DirectionalLightBundle {
        transform: Transform::default().looking_at(Vec3::new(1.0, -4.0, 1.0), Vec3::new(0.0, 1.0, 0.0)),
        directional_light: DirectionalLight {
            illuminance: 5000.0,
            ..Default::default()
        },
        ..Default::default()
    });

    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 1.5, 5.0)),
            ..default()
        },
        FogSettings {
            color: Color::srgb(0.5, 0.5, 0.5),
            falloff: FogFalloff::Linear {
                start: 50.0,
                end: 250.0,
            },
            ..default()
        },
        PanOrbitCamera::default(),
    ));

    // commands.insert_resource(ClearColor(Color::srgb(0.25, 0.25, 0.25)));
}



// fn draw_axis(
//     mut gizmos: Gizmos,
// ) {
//     gizmos.axes(Transform::from_xyz(0.0, 0.0, 0.0), 5.0)
// }



