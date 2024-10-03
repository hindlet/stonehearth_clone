use bevy::prelude::*;

use crate::{maths::{OctaveNoiseGen, SelectiveNoiseGen}, VoxelGrid, VoxelTextureHandle};

use super::TerrainChunkBundle;



pub fn spawn_world_chunks(
    commands: &mut Commands,
    noise_gen: &SelectiveNoiseGen,
    voxel_texture_handle: Handle<StandardMaterial>,
    meshes: &mut ResMut<Assets<Mesh>>,
    diameter: u32,
    depth: u32,
    chunk_side_length: u32,
) {

    let offset = Vec3::new((chunk_side_length * diameter) as f32, 0.0, (chunk_side_length * diameter) as f32) * -0.5;

    for chunk_x in 0..diameter {
        for chunk_z in 0..diameter {
            for chunk_y in 0..depth {

                let chunk_pos = Vec3::new((chunk_x * chunk_side_length) as f32, (chunk_y * chunk_side_length) as f32, (chunk_z * chunk_side_length) as f32);
                let map = noise_gen.gen_map(
                    [chunk_side_length as usize; 2],
                    chunk_pos.xz(),
                    1.0,
                    false,
                    true,
                );

                let mut grid: VoxelGrid = VoxelGrid::empty();
                grid.dims = UVec3::new(chunk_side_length, chunk_side_length, chunk_side_length);
                grid.scale = 1.0;

                for x in 0..chunk_side_length {
                    for z in 0..chunk_side_length {
                        let height = map[(z * chunk_side_length + x) as usize];
                        for y in 0..chunk_side_length {
                            if y as f32 <= height {
                                grid.voxels.insert(UVec3::new(x, y, z), 8);
                            } else {
                                // do not add voxel here
                            }
                        }
                    }
                }



                commands.spawn(TerrainChunkBundle::new(
                    chunk_pos + offset,
                    grid,
                    voxel_texture_handle.clone_weak(),
                    meshes
                ));
            }
        }
    }
}