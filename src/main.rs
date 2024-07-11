use std::{fs::File, io::Write};

use bevy::{color::palettes::css::WHITE, pbr::wireframe::{WireframeConfig, WireframePlugin}, prelude::*, render::{settings::{RenderCreation, WgpuFeatures, WgpuSettings}, RenderPlugin}};
mod building;
mod voxels;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
// use smooth_bevy_cameras::{controllers::fps::{FpsCameraBundle, FpsCameraController, FpsCameraPlugin}, LookTransformPlugin};
use voxels::*;
// mod palette_gen;

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
        .insert_resource(WireframeConfig {
            // The global wireframe config enables drawing of wireframes on every mesh,
            // except those with `NoWireframe`. Meshes with `Wireframe` will always have a wireframe,
            // regardless of the global configuration.
            global: false,
            // Controls the default color of all wireframes. Used as the default color for global wireframes.
            // Can be changed per mesh using the `WireframeColor` component.
            default_color: WHITE.into(),
        })
        .add_plugins((building::BuildingPlugin, WireframePlugin, voxels::VoxelPlugin))
        // .add_plugins((FpsCameraPlugin::default(), LookTransformPlugin))
        .add_plugins(PanOrbitCameraPlugin)
        .add_systems(Startup, spawn_scene_basics)
        .add_systems(Update, toggle_wireframe)
        .add_systems(Update, draw_axis)
        .run();
}



fn toggle_wireframe(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut config: ResMut<WireframeConfig>,
    // mut mesh_visiblity: Query<&mut Visibility, With<Handle<Mesh>>>
) {


    if keyboard_input.just_pressed(KeyCode::Space) {
        config.global ^= true;
        // for mut visibility in mesh_visiblity.iter_mut() {
        //     match visibility.to_owned() {
        //         Visibility::Hidden => *visibility = Visibility::Visible,
        //         Visibility::Visible => *visibility = Visibility::Hidden,
        //         Visibility::Inherited => *visibility = Visibility::Hidden
        //     }
        // }
    }

}

fn spawn_scene_basics(
    mut commands: Commands,
    mut ambient_light: ResMut<AmbientLight>
) {

    ambient_light.brightness = 1000.0;

    // commands
    //     .spawn(Camera3dBundle::default())
    //     .insert(FpsCameraBundle::new(
    //         FpsCameraController::default(),
    //         Vec3::new(-2.0, 5.0, 5.0),
    //         Vec3::new(0., 0., 0.),
    //         Vec3::Y,
    //     ));
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 1.5, 5.0)),
            ..default()
        },
        PanOrbitCamera::default(),
    ));

}



fn draw_axis(
    mut gizmos: Gizmos,
) {
    gizmos.axes(Transform::from_xyz(0.0, 0.0, 0.0), 5.0)
}

// fn read_voxel_test_data() {
//     let vox_data = vox_format::from_file("assets/Test_Scene.vox").unwrap();
//     let text = format!("{:#?}", vox_data);
//     let mut file = File::create("assets/voxel_data.txt").unwrap();
//     let _ = file.write(&text.into_bytes());
// }