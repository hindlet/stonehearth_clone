use bevy::{prelude::*, utils::hashbrown::HashMap};

use crate::voxels::VoxelGrid;

use super::get_voxel_colour_index;




pub enum VoxelScale {
    Terrain,
    Entity
}

fn translate_scale(scale: VoxelScale) -> f32 {
    match scale {
        VoxelScale::Terrain => 1.0,
        VoxelScale::Entity => 0.1,
    }
}



/// Reads the first grid within a .vox file 
/// 
/// Only use if you are sure that the file contains 1 grid or if you do not care about the other ones
pub fn read_vox_file_single(path: &str, scale: VoxelScale) -> VoxelGrid {
    let vox_data = vox_format::from_file(path).unwrap();
    
    let model = &vox_data.models[0];
    let dims = UVec3::new(model.size.x, model.size.z, model.size.y);
    let mut voxels: HashMap<UVec3, u8> = HashMap::new();

    for voxel in model.voxels.iter() {
        let col = vox_data.palette.get(voxel.color_index);
        if let Some(colour_index) = get_voxel_colour_index(col) {
            let pos = UVec3::new(voxel.point.x as u32, voxel.point.z as u32, voxel.point.y as u32);
            voxels.insert(pos, colour_index);
        }
    }

    VoxelGrid {
        scale: translate_scale(scale),
        dims,
        voxels
    }
}


/// reads all the different grids withing a .vox file
pub fn read_vox_file(path: &str, scale: VoxelScale) -> Vec<VoxelGrid>{
    let vox_data = vox_format::from_file(path).unwrap();
    let scale = translate_scale(scale);
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
                scale,
                dims,
                voxels
            })
        }
        models
    };
    models
}