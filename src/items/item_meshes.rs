use bevy::{prelude::*, utils::HashMap};

use crate::{mesh_voxels, read_vox_file_single, VoxelGrid, VoxelMesher, VoxelScale};

use super::item_template::{ItemTemplate, ItemTemplateInfo};


#[derive(Resource, Debug)]
pub struct ItemMeshes(HashMap<ItemTemplateInfo, (Handle<Mesh>, VoxelGrid)>);


impl ItemMeshes {
    pub fn new() -> Self {
        ItemMeshes(HashMap::new())
    }
}


pub fn get_item_mesh(
    meshes: &mut ItemMeshes,
    mesh_assets: &mut Assets<Mesh>,
    template: &ItemTemplateInfo,
) -> Handle<Mesh> {
    if let Some((mesh_handle, _)) = meshes.0.get(template) {
        println!("Item mesh exists");
        return mesh_handle.clone_weak();
    } else {
        println!("Item Mesh Does Not Exist, Creating Mesh");
        let grid = read_vox_file_single(&template.voxel_path, VoxelScale::Entity);
        // makes sense to do fancy since we are unlikely to need to remesh
        let mesh_handle = mesh_assets.add(mesh_voxels(&grid, VoxelMesher::FancyMeshing));
        
        meshes.0.insert(template.clone(), (mesh_handle.clone(), grid));

        return mesh_handle.clone_weak();
    }
}