mod vox_reader;
mod voxel_mesher;
mod voxel_texturing;
mod voxels;
use bevy::prelude::*;

pub use vox_reader::read_vox_file;
pub use voxel_mesher::mesh_voxels;
use voxel_mesher::VoxelMesher;
// use voxel_mesher::test_rectangles;
use voxel_texturing::*;
pub use voxels::*;


pub struct VoxelPlugin;


impl Plugin for VoxelPlugin {
    fn build(&self, app: &mut App) {
        app
            // .add_systems(Startup, test_rectangles)
            .add_systems(Startup, (load_voxel_texture, apply_deferred, load_test_voxel_mesh).chain());
    }
}

fn load_test_voxel_mesh(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    voxel_texture_handle: Res<VoxelTextureHandle>
) {

    let mesh = mesh_voxels(&read_vox_file("assets/Test_Scene.vox")[0], VoxelMesher::FancyMeshing);
    let mesh_handle = meshes.add(mesh);

    commands.spawn(PbrBundle {
        mesh: mesh_handle,
        material: voxel_texture_handle.material_handle.clone(),
        ..Default::default()
    });
}