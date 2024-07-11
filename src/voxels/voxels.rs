use bevy::{math::UVec3, utils::hashbrown::HashMap};
use bevy::prelude::*;




pub enum VoxelType {
    Stone,
    Earth,
    Grass
}


/// For things that are not editable, such as creatures and stuff idk
/// 1.0 scale -> 1 voxel per meter
#[derive(Component)]
pub struct VoxelGrid {
    pub scale: f32,
    pub dims: UVec3,
    pub voxels: HashMap<UVec3, u8>
}

impl VoxelGrid {
    pub fn empty() -> Self {
        Self {
            scale: 1.0,
            dims: UVec3::ZERO,
            voxels: HashMap::new()
        }
    }
}

/// For things that are editable, like terrain and shit like that
/// /// 1.0 scale -> 1 voxel per meter
#[derive(Component)]
pub struct InteractableVoxelGrid {
    pub scale: f32,
    pub dims: UVec3,
    pub voxels: HashMap<UVec3, (VoxelType, u8)>
}