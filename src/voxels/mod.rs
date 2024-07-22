mod vox_reader;
mod voxel_mesher;
mod voxel_texturing;
mod voxels;
use std::time::Instant;

use bevy::prelude::*;

pub use vox_reader::read_vox_file;
pub use voxel_mesher::*;
// use voxel_mesher::test_rectangles;
use voxel_texturing::*;
pub use voxels::*;
mod mesh_upgrade;
use mesh_upgrade::MeshUpgradePlugin;
pub use mesh_upgrade::MeshUpgradeStatus;
pub use voxel_texturing::VoxelTextureHandle;



pub struct VoxelPlugin;


impl Plugin for VoxelPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(MeshUpgradePlugin)
            .add_systems(Startup, load_voxel_texture);
            // .add_systems(Startup, (load_voxel_texture, apply_deferred, load_test_voxel_mesh).chain());
            
    }
}

fn load_test_voxel_mesh(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    voxel_texture_handle: Res<VoxelTextureHandle>
) {

    let grid = read_vox_file("assets/Test_Scene.vox")[0].clone();

    commands.spawn(PbrBundle {
        material: voxel_texture_handle.material_handle.clone_weak(),
        mesh: meshes.add(mesh_voxels(&grid, VoxelMesher::NaiveMeshing)),
        ..Default::default()
    })
        .insert(grid)
        .insert(MeshUpgradeStatus::NaiveDefault(Instant::now()));
}