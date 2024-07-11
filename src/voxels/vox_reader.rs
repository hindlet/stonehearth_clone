use bevy::{prelude::*, utils::hashbrown::HashMap};

use crate::voxels::VoxelGrid;

use super::get_voxel_colour_index;






pub fn read_vox_file(path: &str) -> Vec<VoxelGrid>{
    let vox_data = vox_format::from_file(path).unwrap();
    let models = {
        let mut models: Vec<VoxelGrid> = Vec::new();
        for model in vox_data.models.iter() {

            let dims = UVec3::new(model.size.x, model.size.z, model.size.y);
            let mut voxels: HashMap<UVec3, u8> = HashMap::new();


            for voxel in model.voxels.iter() {
                let col = vox_data.palette.get(voxel.color_index);
                if let Some(colour_index) = get_voxel_colour_index(col) {
                    let pos = UVec3::new(voxel.point.x as u32, voxel.point.z as u32, voxel.point.y as u32);
                    voxels.insert(pos, colour_index);
                }
            }

            models.push(VoxelGrid {
                scale: 1.0,
                dims,
                voxels
            })
        }
        models
    };
    models
}