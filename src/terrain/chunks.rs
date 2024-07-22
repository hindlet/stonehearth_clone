use std::time::{Duration, Instant};
use bevy::prelude::*;
use crate::{mesh_voxels, MeshUpgradeStatus, VoxelGrid};




#[derive(Bundle)]
pub struct TerrainChunkBundle {
    tag: TerrainChunkTag,
    mesh_status: MeshUpgradeStatus,
    pbr_bundle: PbrBundle,
    voxel_grid: VoxelGrid
}

impl TerrainChunkBundle {
    pub fn new(
        pos: Vec3,
        grid: VoxelGrid,
        voxel_mat_handle: Handle<StandardMaterial>,
        meshes: &mut ResMut<Assets<Mesh>>
    ) -> Self {

        let mesh = mesh_voxels(&grid, crate::VoxelMesher::NaiveMeshing);

        TerrainChunkBundle {
            tag: TerrainChunkTag,
            mesh_status: MeshUpgradeStatus::NaiveDefault(Instant::now()),
            pbr_bundle: PbrBundle {
                mesh: meshes.add(mesh),
                material: voxel_mat_handle,
                transform: Transform::from_translation(pos),
                ..Default::default()
            },
            voxel_grid: grid
        }

    }
}



#[derive(Component)]
pub struct TerrainChunkTag;






