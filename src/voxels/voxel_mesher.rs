use bevy::{prelude::*, render::{mesh::Indices, render_asset::RenderAssetUsages}, utils::HashMap};
use crate::maths::quicksort_descending;
use super::{get_voxel_colour_uv, VoxelGrid};



pub enum VoxelMesher {
    ShitMeshing,
    NaiveMeshing,
    FancyMeshing,
}

/// Data from Test_Scene.vox:
/// - Shit_Meshing: 0.00172426 seconds - 10560 triangles
/// - Naive Meshing: 0.00242973 seconds - 3788 triangles
/// - Fancy_Meshing: 0.042495526 seconds - 340 triangles
pub fn mesh_voxels(
    grid: &VoxelGrid,
    mesher: VoxelMesher
) -> Mesh{

    match mesher {
        VoxelMesher::ShitMeshing => return shit_meshing(grid),
        VoxelMesher::NaiveMeshing => return naive_meshing(grid),
        VoxelMesher::FancyMeshing => return fancy_meshing(grid)
    }
}


fn shit_meshing(
    grid: &VoxelGrid
) -> Mesh {
    // let start_time = Instant::now();

    let mut positions: Vec<[f32; 3]> = Vec::new();
    let mut normals: Vec<[f32; 3]> = Vec::new();
    let mut triangle_indices: Vec<u32> = Vec::new();
    let mut uvs: Vec<[f32; 2]> = Vec::new();

    // add many cubes OH GOD SO MANY VERTICES
    for (pos, colour_index) in grid.voxels.iter() {
        let index_offset = positions.len() as u32;
        let pos = Vec3::new(pos.x as f32, pos.y as f32, pos.z as f32);
        let uv = get_voxel_colour_uv(*colour_index);
        for _ in 0..24 {
            uvs.push(uv);
        }
        // top(+y)
        positions.push((pos + Vec3::new(-0.5, 0.5, -0.5)).into());
        positions.push((pos + Vec3::new(0.5, 0.5, -0.5)).into());
        positions.push((pos + Vec3::new(0.5, 0.5, 0.5)).into());
        positions.push((pos + Vec3::new(-0.5, 0.5, 0.5)).into());
        normals.push([0.0, 1.0, 0.0]);
        normals.push([0.0, 1.0, 0.0]);
        normals.push([0.0, 1.0, 0.0]);
        normals.push([0.0, 1.0, 0.0]);
        triangle_indices.push(index_offset + 0);
        triangle_indices.push(index_offset + 3);
        triangle_indices.push(index_offset + 1);
        triangle_indices.push(index_offset + 1);
        triangle_indices.push(index_offset + 3);
        triangle_indices.push(index_offset + 2);
        // bottom(-y)
        positions.push((pos + Vec3::new(-0.5, -0.5, -0.5)).into());
        positions.push((pos + Vec3::new(0.5, -0.5, -0.5)).into());
        positions.push((pos + Vec3::new(0.5, -0.5, 0.5)).into());
        positions.push((pos + Vec3::new(-0.5, -0.5, 0.5)).into());
        normals.push([0.0, -1.0, 0.0]);
        normals.push([0.0, -1.0, 0.0]);
        normals.push([0.0, -1.0, 0.0]);
        normals.push([0.0, -1.0, 0.0]);
        triangle_indices.push(index_offset + 4);
        triangle_indices.push(index_offset + 5);
        triangle_indices.push(index_offset + 7);
        triangle_indices.push(index_offset + 5);
        triangle_indices.push(index_offset + 6);
        triangle_indices.push(index_offset + 7);
        // right(+x)
        positions.push((pos + Vec3::new(0.5, -0.5, -0.5)).into());
        positions.push((pos + Vec3::new(0.5, -0.5, 0.5)).into());
        positions.push((pos + Vec3::new(0.5, 0.5, 0.5)).into());
        positions.push((pos + Vec3::new(0.5, 0.5, -0.5)).into());
        normals.push([1.0, 0.0, 0.0]);
        normals.push([1.0, 0.0, 0.0]);
        normals.push([1.0, 0.0, 0.0]);
        normals.push([1.0, 0.0, 0.0]);
        triangle_indices.push(index_offset + 8);
        triangle_indices.push(index_offset + 11);
        triangle_indices.push(index_offset + 9);
        triangle_indices.push(index_offset + 9);
        triangle_indices.push(index_offset + 11);
        triangle_indices.push(index_offset + 10);
        // left(-x)
        positions.push((pos + Vec3::new(-0.5, -0.5, -0.5)).into());
        positions.push((pos + Vec3::new(-0.5, -0.5, 0.5)).into());
        positions.push((pos + Vec3::new(-0.5, 0.5, 0.5)).into());
        positions.push((pos + Vec3::new(-0.5, 0.5, -0.5)).into());
        normals.push([-1.0, 0.0, 0.0]);
        normals.push([-1.0, 0.0, 0.0]);
        normals.push([-1.0, 0.0, 0.0]);
        normals.push([-1.0, 0.0, 0.0]);
        triangle_indices.push(index_offset + 12);
        triangle_indices.push(index_offset + 13);
        triangle_indices.push(index_offset + 15);
        triangle_indices.push(index_offset + 13);
        triangle_indices.push(index_offset + 14);
        triangle_indices.push(index_offset + 15);
        // back(+z)
        positions.push((pos + Vec3::new(-0.5, -0.5, 0.5)).into());
        positions.push((pos + Vec3::new(-0.5, 0.5, 0.5)).into());
        positions.push((pos + Vec3::new(0.5, 0.5, 0.5)).into());
        positions.push((pos + Vec3::new(0.5, -0.5, 0.5)).into());
        normals.push([0.0, 0.0, 1.0]);
        normals.push([0.0, 0.0, 1.0]);
        normals.push([0.0, 0.0, 1.0]);
        normals.push([0.0, 0.0, 1.0]);
        triangle_indices.push(index_offset + 16);
        triangle_indices.push(index_offset + 19);
        triangle_indices.push(index_offset + 17);
        triangle_indices.push(index_offset + 17);
        triangle_indices.push(index_offset + 19);
        triangle_indices.push(index_offset + 18);
        // forward(-z)
        positions.push((pos + Vec3::new(-0.5, -0.5, -0.5)).into());
        positions.push((pos + Vec3::new(-0.5, 0.5, -0.5)).into());
        positions.push((pos + Vec3::new(0.5, 0.5, -0.5)).into());
        positions.push((pos + Vec3::new(0.5, -0.5, -0.5)).into());
        normals.push([0.0, 0.0, -1.0]);
        normals.push([0.0, 0.0, -1.0]);
        normals.push([0.0, 0.0, -1.0]);
        normals.push([0.0, 0.0, -1.0]);
        triangle_indices.push(index_offset + 20);
        triangle_indices.push(index_offset + 21);
        triangle_indices.push(index_offset + 23);
        triangle_indices.push(index_offset + 21);
        triangle_indices.push(index_offset + 22);
        triangle_indices.push(index_offset + 23);
    }

    // println!("Shit_Meshing: {} seconds - {} triangles", start_time.elapsed().as_secs_f32(), triangle_indices.len() / 3);

    let mesh = Mesh::new(bevy::render::mesh::PrimitiveTopology::TriangleList, RenderAssetUsages::all())
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
        .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
        .with_inserted_indices(Indices::U32(triangle_indices));
    mesh
}

fn naive_meshing(
    grid: &VoxelGrid
) -> Mesh {
    // let start_time = Instant::now();

    let mut positions: Vec<[f32; 3]> = Vec::new();
    let mut normals: Vec<[f32; 3]> = Vec::new();
    let mut triangle_indices: Vec<u32> = Vec::new();
    let mut uvs: Vec<[f32; 2]> = Vec::new();

    // add many cubes OH GOD SO MANY VERTICES
    for (pos, colour_index) in grid.voxels.iter() {
        let sides = (
            !grid.voxels.contains_key(&(*pos + UVec3::Y)),
            pos.y == 0 || !grid.voxels.contains_key(&(*pos - UVec3::Y)),
            !grid.voxels.contains_key(&(*pos + UVec3::X)),
            pos.x == 0 || !grid.voxels.contains_key(&(*pos - UVec3::X)),
            !grid.voxels.contains_key(&(*pos + UVec3::Z)),
            pos.z == 0 || !grid.voxels.contains_key(&(*pos - UVec3::Z)),
        );
        let pos = Vec3::new(pos.x as f32, pos.y as f32, pos.z as f32);
        let uv = get_voxel_colour_uv(*colour_index);


        let mut index_offset = positions.len() as u32;
        // top(+y)
        if sides.0 {
            positions.push((pos + Vec3::new(-0.5, 0.5, -0.5)).into());
            positions.push((pos + Vec3::new(0.5, 0.5, -0.5)).into());
            positions.push((pos + Vec3::new(0.5, 0.5, 0.5)).into());
            positions.push((pos + Vec3::new(-0.5, 0.5, 0.5)).into());
            normals.push([0.0, 1.0, 0.0]);
            normals.push([0.0, 1.0, 0.0]);
            normals.push([0.0, 1.0, 0.0]);
            normals.push([0.0, 1.0, 0.0]);
            uvs.push(uv);
            uvs.push(uv);
            uvs.push(uv);
            uvs.push(uv);
            triangle_indices.push(index_offset + 0);
            triangle_indices.push(index_offset + 3);
            triangle_indices.push(index_offset + 1);
            triangle_indices.push(index_offset + 1);
            triangle_indices.push(index_offset + 3);
            triangle_indices.push(index_offset + 2);
            index_offset += 4;
        }
        // bottom(-y)
        if sides.1 {
            positions.push((pos + Vec3::new(-0.5, -0.5, -0.5)).into());
            positions.push((pos + Vec3::new(0.5, -0.5, -0.5)).into());
            positions.push((pos + Vec3::new(0.5, -0.5, 0.5)).into());
            positions.push((pos + Vec3::new(-0.5, -0.5, 0.5)).into());
            normals.push([0.0, -1.0, 0.0]);
            normals.push([0.0, -1.0, 0.0]);
            normals.push([0.0, -1.0, 0.0]);
            normals.push([0.0, -1.0, 0.0]);
            uvs.push(uv);
            uvs.push(uv);
            uvs.push(uv);
            uvs.push(uv);
            triangle_indices.push(index_offset + 0);
            triangle_indices.push(index_offset + 1);
            triangle_indices.push(index_offset + 3);
            triangle_indices.push(index_offset + 1);
            triangle_indices.push(index_offset + 2);
            triangle_indices.push(index_offset + 3);
            index_offset += 4;
        }
        // right(+x)
        if sides.2 {
            positions.push((pos + Vec3::new(0.5, -0.5, -0.5)).into());
            positions.push((pos + Vec3::new(0.5, -0.5, 0.5)).into());
            positions.push((pos + Vec3::new(0.5, 0.5, 0.5)).into());
            positions.push((pos + Vec3::new(0.5, 0.5, -0.5)).into());
            normals.push([1.0, 0.0, 0.0]);
            normals.push([1.0, 0.0, 0.0]);
            normals.push([1.0, 0.0, 0.0]);
            normals.push([1.0, 0.0, 0.0]);
            uvs.push(uv);
            uvs.push(uv);
            uvs.push(uv);
            uvs.push(uv);
            triangle_indices.push(index_offset + 0);
            triangle_indices.push(index_offset + 3);
            triangle_indices.push(index_offset + 1);
            triangle_indices.push(index_offset + 1);
            triangle_indices.push(index_offset + 3);
            triangle_indices.push(index_offset + 2);
            index_offset += 4;
        }
        // left(-x)
        if sides.3 {
            positions.push((pos + Vec3::new(-0.5, -0.5, -0.5)).into());
            positions.push((pos + Vec3::new(-0.5, -0.5, 0.5)).into());
            positions.push((pos + Vec3::new(-0.5, 0.5, 0.5)).into());
            positions.push((pos + Vec3::new(-0.5, 0.5, -0.5)).into());
            normals.push([-1.0, 0.0, 0.0]);
            normals.push([-1.0, 0.0, 0.0]);
            normals.push([-1.0, 0.0, 0.0]);
            normals.push([-1.0, 0.0, 0.0]);
            uvs.push(uv);
            uvs.push(uv);
            uvs.push(uv);
            uvs.push(uv);
            triangle_indices.push(index_offset + 0);
            triangle_indices.push(index_offset + 1);
            triangle_indices.push(index_offset + 3);
            triangle_indices.push(index_offset + 1);
            triangle_indices.push(index_offset + 2);
            triangle_indices.push(index_offset + 3);
            index_offset += 4;
        }
        // back(+z)
        if sides.4 {
            positions.push((pos + Vec3::new(-0.5, -0.5, 0.5)).into());
            positions.push((pos + Vec3::new(-0.5, 0.5, 0.5)).into());
            positions.push((pos + Vec3::new(0.5, 0.5, 0.5)).into());
            positions.push((pos + Vec3::new(0.5, -0.5, 0.5)).into());
            normals.push([0.0, 0.0, 1.0]);
            normals.push([0.0, 0.0, 1.0]);
            normals.push([0.0, 0.0, 1.0]);
            normals.push([0.0, 0.0, 1.0]);
            uvs.push(uv);
            uvs.push(uv);
            uvs.push(uv);
            uvs.push(uv);
            triangle_indices.push(index_offset + 0);
            triangle_indices.push(index_offset + 3);
            triangle_indices.push(index_offset + 1);
            triangle_indices.push(index_offset + 1);
            triangle_indices.push(index_offset + 3);
            triangle_indices.push(index_offset + 2);
            index_offset += 4;
        }
        // forward(-z)
        if sides.5 {
            positions.push((pos + Vec3::new(-0.5, -0.5, -0.5)).into());
            positions.push((pos + Vec3::new(-0.5, 0.5, -0.5)).into());
            positions.push((pos + Vec3::new(0.5, 0.5, -0.5)).into());
            positions.push((pos + Vec3::new(0.5, -0.5, -0.5)).into());
            normals.push([0.0, 0.0, -1.0]);
            normals.push([0.0, 0.0, -1.0]);
            normals.push([0.0, 0.0, -1.0]);
            normals.push([0.0, 0.0, -1.0]);
            uvs.push(uv);
            uvs.push(uv);
            uvs.push(uv);
            uvs.push(uv);
            triangle_indices.push(index_offset + 0);
            triangle_indices.push(index_offset + 1);
            triangle_indices.push(index_offset + 3);
            triangle_indices.push(index_offset + 1);
            triangle_indices.push(index_offset + 2);
            triangle_indices.push(index_offset + 3);
        }
    }


    // println!("Naive Meshing: {} seconds - {} triangles", start_time.elapsed().as_secs_f32(), triangle_indices.len() / 3);

    let mesh = Mesh::new(bevy::render::mesh::PrimitiveTopology::TriangleList, RenderAssetUsages::all())
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
        .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
        .with_inserted_indices(Indices::U32(triangle_indices));
    mesh
}



// costly upfront but very fast overall
// TODO, up vs down faces and remove voxels that are covered
fn fancy_meshing(
    grid: &VoxelGrid
) -> Mesh{

    // let start_time = Instant::now();

    // REMEMBER: GRID DIMS: (x, z, y) I DO NOT KNOW WHY AAAAAAAAA
    let mut x_slices: Vec<(u32, u32, u32, u8, Vec<bool>, Vec<bool>)> = Vec::new(); // x-cord, up/down, width, height, colour_id, pos_slice, neg_slice
    let mut y_slices: Vec<(u32, u32, u32, u8, Vec<bool>, Vec<bool>)> = Vec::new(); // y-cord, up/down, width, height, colour_id, pos_slice, neg_slice
    let mut z_slices: Vec<(u32, u32, u32, u8, Vec<bool>, Vec<bool>)> = Vec::new(); // z-cord, up/down,  width, height, colour_id, pos_slice, neg_slice

    // generate all the x slices
    for x in 0..grid.dims.x {
        let mut slices: HashMap<u8, (u32, u32, u32, Vec<bool>, Vec<bool>)> = HashMap::new();

        for y in 0..grid.dims.y {
            for z in 0..grid.dims.z {
                let pos = UVec3::new(x, y, z);
                if let Some(colour_id) = grid.voxels.get(&pos) {
                    // print!("{:?}    ", colour_id);
                    let mut found_id = false;
                    for (slice_colour_id, data) in slices.iter_mut() {
                        if slice_colour_id == colour_id {
                            found_id = true;
                            data.3.push(x == grid.dims.x - 1 || !grid.voxels.contains_key(&(pos + UVec3::X)));
                            data.4.push(x == 0 || !grid.voxels.contains_key(&(pos - UVec3::X)));
                            // println!("FOUND")
                        } else {
                            data.3.push(false);
                            data.4.push(false);
                        }
                    }
                    if !found_id {
                        // println!("NOT FOUND");
                        let index = y * grid.dims.z + z;
                        let mut initial_data_pos = Vec::new();
                        for _ in 0..index {
                            initial_data_pos.push(false)
                        }
                        let mut initial_data_neg = initial_data_pos.clone();

                        initial_data_pos.push(x == grid.dims.x - 1 || !grid.voxels.contains_key(&(pos + UVec3::X)));
                        initial_data_neg.push(x == 0 || !grid.voxels.contains_key(&(pos - UVec3::X)));
                        slices.insert(*colour_id, (x, grid.dims.z, grid.dims.y, initial_data_pos, initial_data_neg));
                    }
                } else {
                    // println!("No Colour ID");
                    for (_, data) in slices.iter_mut() {
                        data.3.push(false);
                        data.4.push(false);
                    }
                }
            }
        }

        for (colour_id, (x_pos, width, height, pos_grid, neg_grid)) in slices {
            x_slices.push((x_pos, width, height, colour_id, pos_grid, neg_grid));
        }
    }

    // generate all the z slices
    for z in 0..grid.dims.z {
        let mut slices: HashMap<u8, (u32, u32, u32, Vec<bool>, Vec<bool>)> = HashMap::new();

        for y in 0..grid.dims.y {
            for x in 0..grid.dims.x {
                let pos = UVec3::new(x, y, z);
                if let Some(colour_id) = grid.voxels.get(&pos) {
                    // print!("{:?}    ", colour_id);
                    let mut found_id = false;
                    for (slice_colour_id, data) in slices.iter_mut() {
                        if slice_colour_id == colour_id {
                            found_id = true;
                            data.3.push(z == grid.dims.z - 1 || !grid.voxels.contains_key(&(pos + UVec3::Z)));
                            data.4.push(z == 0 || !grid.voxels.contains_key(&(pos - UVec3::Z)));
                            // println!("FOUND")
                        } else {
                            data.3.push(false);
                            data.4.push(false);
                        }
                    }
                    if !found_id {
                        // println!("NOT FOUND");
                        let index = y * grid.dims.x + x;
                        let mut initial_data_pos = Vec::new();
                        for _ in 0..index {
                            initial_data_pos.push(false)
                        }
                        let mut initial_data_neg = initial_data_pos.clone();

                        initial_data_pos.push(z == grid.dims.z - 1 || !grid.voxels.contains_key(&(pos + UVec3::Z)));
                        initial_data_neg.push(z == 0 || !grid.voxels.contains_key(&(pos - UVec3::Z)));
                        slices.insert(*colour_id, (z, grid.dims.x, grid.dims.y, initial_data_pos, initial_data_neg));
                    }
                } else {
                    // println!("No Colour ID");
                    for (_, data) in slices.iter_mut() {
                        data.3.push(false);
                        data.4.push(false);
                    }
                }
            }
        }

        for (colour_id, (z_pos, width, height, pos_grid, neg_grid)) in slices {
            z_slices.push((z_pos, width, height, colour_id, pos_grid, neg_grid));
        }
    }

    // generate all the y slices
    for y in 0..grid.dims.y {
        let mut slices: HashMap<u8, (u32, u32, u32, Vec<bool>, Vec<bool>)> = HashMap::new();

        for z in 0..grid.dims.z {
            for x in 0..grid.dims.x {
                let pos = UVec3::new(x, y, z);
                if let Some(colour_id) = grid.voxels.get(&pos) {
                    // print!("{:?}    ", colour_id);
                    let mut found_id = false;
                    for (slice_colour_id, data) in slices.iter_mut() {
                        if slice_colour_id == colour_id {
                            found_id = true;
                            data.3.push(y == grid.dims.y - 1 || !grid.voxels.contains_key(&(pos + UVec3::Y)));
                            data.4.push(y == 0 || !grid.voxels.contains_key(&(pos - UVec3::Y)));
                            // println!("FOUND")
                        } else {
                            data.3.push(false);
                            data.4.push(false);
                        }
                    }
                    if !found_id {
                        // println!("NOT FOUND");
                        let index = z * grid.dims.x + x;
                        let mut initial_data_pos = Vec::new();
                        for _ in 0..index {
                            initial_data_pos.push(false)
                        }
                        let mut initial_data_neg = initial_data_pos.clone();

                        initial_data_pos.push(y == grid.dims.y - 1 || !grid.voxels.contains_key(&(pos + UVec3::Y)));
                        initial_data_neg.push(y == 0 || !grid.voxels.contains_key(&(pos - UVec3::Y)));
                        slices.insert(*colour_id, (y, grid.dims.x, grid.dims.z, initial_data_pos, initial_data_neg));
                    }
                } else {
                    // println!("No Colour ID");
                    for (_, data) in slices.iter_mut() {
                        data.3.push(false);
                        data.4.push(false);
                    }
                }
            }
        }

        for (colour_id, (y_pos, width, height, pos_grid, neg_grid)) in slices {
            y_slices.push((y_pos, width, height, colour_id, pos_grid, neg_grid));
        }
    }

    // test print
    // for slice in x_slices.iter() {
    //     if slice.0 == 3 {
    //         let coords = get_voxel_colour_uv(slice.3);
    //         println!("Colour id: {:?}, Colour coords: [{:?}, {:?}]", slice.3, coords[0] * 15.0, coords[1] * 15.0);
    //         println!("{:?} \n \n", slice.4);
    //     }
    // }


    let mut positions: Vec<[f32; 3]> = Vec::new();
    let mut normals: Vec<[f32; 3]> = Vec::new();
    let mut triangle_indices: Vec<u32> = Vec::new();
    let mut uvs: Vec<[f32; 2]> = Vec::new();

    // okay now for each slice generate a series of rectangles
    // and from that make a mesh

    let mut index_offset = 0;

    // for (_, width, height, _, pos_grid, neg_grid) in x_slices.iter() {
    //     println!("{} : {}, {}", width * height, pos_grid.len(), neg_grid.len());
    // }
    // println!("\n\n\n\n\n");
    // for (_, width, height, _, pos_grid, neg_grid) in z_slices.iter() {
    //     println!("{} : {}, {}", width * height, pos_grid.len(), neg_grid.len());
    // }
    // println!("\n\n\n\n\n");
    // for (y_pos, _, _, colour_id, pos_grid, neg_grid) in y_slices.iter() {
    //     if y_pos == &0 && colour_id == &193 {
    //         for i in 0..pos_grid.len() {
    //             if pos_grid[i] != neg_grid[i] {println!("{}", i)}
    //         }
    //         println!("{} : {}", pos_grid.len(), neg_grid.len());
    //         for i in 0..24 {
    //             for j in 0..24 {
    //                 print!("{}-", pos_grid[i * 24 + j]);
    //             }
    //             println!("")
    //         }
    //         // println!("{} : \n{:?} \n{:?}", colour_id, pos_grid, neg_grid);
    //     }
    // }
    // println!("\n\n\n\n\n");

    for (x_pos, width, height, colour_id, pos_grid, neg_grid) in x_slices {
        let uv = get_voxel_colour_uv(colour_id);
        
        // pos grid
        for (z_pos, y_pos, z_len, y_len) in break_into_rects(&pos_grid, width, height) {
            let x_pos = grid.scale * (x_pos as f32 + 0.5);
            positions.push([
                x_pos,
                grid.scale * (y_pos as f32 - 0.5),
                grid.scale * (z_pos as f32 - 0.5),
            ]);
            positions.push([
                x_pos,
                grid.scale * (y_pos as f32 - 0.5),
                grid.scale * ((z_pos + z_len) as f32 - 0.5),
            ]);
            positions.push([
                x_pos,
                grid.scale * ((y_pos + y_len) as f32 - 0.5),
                grid.scale * ((z_pos + z_len) as f32 - 0.5),
            ]);
            positions.push([
                x_pos,
                grid.scale * ((y_pos + y_len) as f32 - 0.5),
                grid.scale * (z_pos as f32 - 0.5),
            ]);
            
            uvs.push(uv);
            uvs.push(uv);
            uvs.push(uv);
            uvs.push(uv);
            normals.push([1.0, 0.0, 0.0]);
            normals.push([1.0, 0.0, 0.0]);
            normals.push([1.0, 0.0, 0.0]);
            normals.push([1.0, 0.0, 0.0]);
            triangle_indices.push(index_offset + 0);
            triangle_indices.push(index_offset + 3);
            triangle_indices.push(index_offset + 1);
            triangle_indices.push(index_offset + 1);
            triangle_indices.push(index_offset + 3);
            triangle_indices.push(index_offset + 2);
            // println!("{}", index_offset);
            index_offset += 4;
        }
        // neg grid
        for (z_pos, y_pos, z_len, y_len) in break_into_rects(&neg_grid, width, height) {
            let x_pos = grid.scale * (x_pos as f32 - 0.5);
            positions.push([
                x_pos,
                grid.scale * (y_pos as f32 - 0.5),
                grid.scale * (z_pos as f32 - 0.5),
            ]);
            positions.push([
                x_pos,
                grid.scale * (y_pos as f32 - 0.5),
                grid.scale * ((z_pos + z_len) as f32 - 0.5),
            ]);
            positions.push([
                x_pos,
                grid.scale * ((y_pos + y_len) as f32 - 0.5),
                grid.scale * ((z_pos + z_len) as f32 - 0.5),
            ]);
            positions.push([
                x_pos,
                grid.scale * ((y_pos + y_len) as f32 - 0.5),
                grid.scale * (z_pos as f32 - 0.5),
            ]);
            
            uvs.push(uv);
            uvs.push(uv);
            uvs.push(uv);
            uvs.push(uv);
            normals.push([-1.0, 0.0, 0.0]);
            normals.push([-1.0, 0.0, 0.0]);
            normals.push([-1.0, 0.0, 0.0]);
            normals.push([-1.0, 0.0, 0.0]);
            triangle_indices.push(index_offset + 0);
            triangle_indices.push(index_offset + 1);
            triangle_indices.push(index_offset + 3);
            triangle_indices.push(index_offset + 1);
            triangle_indices.push(index_offset + 2);
            triangle_indices.push(index_offset + 3);
            // println!("{}", index_offset);
            index_offset += 4;
        }
    }

    for (z_pos, width, height, colour_id, pos_grid, neg_grid) in z_slices {
        let uv = get_voxel_colour_uv(colour_id);
        
        // pos grid
        for (x_pos, y_pos, x_len, y_len) in break_into_rects(&pos_grid, width, height) {
            let z_pos = grid.scale * (z_pos as f32 + 0.5);
            positions.push([
                grid.scale * (x_pos as f32 - 0.5),
                grid.scale * (y_pos as f32 - 0.5),
                z_pos,
            ]);
            positions.push([
                grid.scale * (x_pos as f32 - 0.5),
                grid.scale * ((y_pos + y_len) as f32 - 0.5),
                z_pos,
            ]);
            positions.push([
                grid.scale * ((x_pos + x_len) as f32 - 0.5),
                grid.scale * ((y_pos + y_len) as f32 - 0.5),
                z_pos,
            ]);
            positions.push([
                grid.scale * ((x_pos + x_len) as f32 - 0.5),
                grid.scale * (y_pos as f32 - 0.5),
                z_pos,
            ]);
            
            uvs.push(uv);
            uvs.push(uv);
            uvs.push(uv);
            uvs.push(uv);
            normals.push([0.0, 0.0, 1.0]);
            normals.push([0.0, 0.0, 1.0]);
            normals.push([0.0, 0.0, 1.0]);
            normals.push([0.0, 0.0, 1.0]);
            triangle_indices.push(index_offset + 0);
            triangle_indices.push(index_offset + 3);
            triangle_indices.push(index_offset + 1);
            triangle_indices.push(index_offset + 1);
            triangle_indices.push(index_offset + 3);
            triangle_indices.push(index_offset + 2);
            // println!("{}", index_offset);
            index_offset += 4;
        }
        // neg grid
        for (x_pos, y_pos, x_len, y_len) in break_into_rects(&neg_grid, width, height) {
            let z_pos = grid.scale * (z_pos as f32 - 0.5);
            positions.push([
                grid.scale * (x_pos as f32 - 0.5),
                grid.scale * (y_pos as f32 - 0.5),
                z_pos,
            ]);
            positions.push([
                grid.scale * (x_pos as f32 - 0.5),
                grid.scale * ((y_pos + y_len) as f32 - 0.5),
                z_pos,
            ]);
            positions.push([
                grid.scale * ((x_pos + x_len) as f32 - 0.5),
                grid.scale * ((y_pos + y_len) as f32 - 0.5),
                z_pos,
            ]);
            positions.push([
                grid.scale * ((x_pos + x_len) as f32 - 0.5),
                grid.scale * (y_pos as f32 - 0.5),
                z_pos,
            ]);
            
            uvs.push(uv);
            uvs.push(uv);
            uvs.push(uv);
            uvs.push(uv);
            normals.push([0.0, 0.0, -1.0]);
            normals.push([0.0, 0.0, -1.0]);
            normals.push([0.0, 0.0, -1.0]);
            normals.push([0.0, 0.0, -1.0]);
            triangle_indices.push(index_offset + 0);
            triangle_indices.push(index_offset + 1);
            triangle_indices.push(index_offset + 3);
            triangle_indices.push(index_offset + 1);
            triangle_indices.push(index_offset + 2);
            triangle_indices.push(index_offset + 3);
            // println!("{}", index_offset);
            index_offset += 4;
        }
    }

    for (y_pos, width, height, colour_id, pos_grid, neg_grid) in y_slices {
        let uv = get_voxel_colour_uv(colour_id);
        
        // pos grid
        for (x_pos, z_pos, x_len, z_len) in break_into_rects(&pos_grid, width, height) {
            let y_pos = grid.scale * (y_pos as f32 + 0.5);
            positions.push([
                grid.scale * (x_pos as f32 - 0.5),
                y_pos,
                grid.scale * (z_pos as f32 - 0.5),
            ]);
            positions.push([
                grid.scale * ((x_pos + x_len) as f32 - 0.5),
                y_pos,
                grid.scale * (z_pos as f32 - 0.5),
            ]);
            positions.push([
                grid.scale * ((x_pos + x_len) as f32 - 0.5),
                y_pos,
                grid.scale * ((z_pos + z_len) as f32 - 0.5),
            ]);
            positions.push([
                grid.scale * (x_pos as f32 - 0.5),
                y_pos,
                grid.scale * ((z_pos + z_len) as f32 - 0.5),
            ]);
            
            uvs.push(uv);
            uvs.push(uv);
            uvs.push(uv);
            uvs.push(uv);
            normals.push([0.0, 1.0, 0.0]);
            normals.push([0.0, 1.0, 0.0]);
            normals.push([0.0, 1.0, 0.0]);
            normals.push([0.0, 1.0, 0.0]);
            triangle_indices.push(index_offset + 0);
            triangle_indices.push(index_offset + 3);
            triangle_indices.push(index_offset + 1);
            triangle_indices.push(index_offset + 1);
            triangle_indices.push(index_offset + 3);
            triangle_indices.push(index_offset + 2);
            // println!("{}", index_offset);
            index_offset += 4;
        }
        // neg grid
        for (x_pos, z_pos, x_len, z_len) in break_into_rects(&neg_grid, width, height) {
            let y_pos = grid.scale * (y_pos as f32 - 0.5);
            positions.push([
                grid.scale * (x_pos as f32 - 0.5),
                y_pos,
                grid.scale * (z_pos as f32 - 0.5),
            ]);
            positions.push([
                grid.scale * ((x_pos + x_len) as f32 - 0.5),
                y_pos,
                grid.scale * (z_pos as f32 - 0.5),
            ]);
            positions.push([
                grid.scale * ((x_pos + x_len) as f32 - 0.5),
                y_pos,
                grid.scale * ((z_pos + z_len) as f32 - 0.5),
            ]);
            positions.push([
                grid.scale * (x_pos as f32 - 0.5),
                y_pos,
                grid.scale * ((z_pos + z_len) as f32 - 0.5),
            ]);
            
            uvs.push(uv);
            uvs.push(uv);
            uvs.push(uv);
            uvs.push(uv);
            normals.push([0.0, -1.0, 0.0]);
            normals.push([0.0, -1.0, 0.0]);
            normals.push([0.0, -1.0, 0.0]);
            normals.push([0.0, -1.0, 0.0]);
            triangle_indices.push(index_offset + 0);
            triangle_indices.push(index_offset + 1);
            triangle_indices.push(index_offset + 3);
            triangle_indices.push(index_offset + 1);
            triangle_indices.push(index_offset + 2);
            triangle_indices.push(index_offset + 3);
            // println!("{}", index_offset);
            index_offset += 4;
        }
    }


    // println!("Fancy_Meshing: {} seconds - {} triangles", start_time.elapsed().as_secs_f32(), triangle_indices.len() / 3);

    Mesh::new(bevy::render::mesh::PrimitiveTopology::TriangleList, RenderAssetUsages::all())
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
        .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
        .with_inserted_indices(Indices::U32(triangle_indices))
}

// pub fn test_rectangles() {
//     let grid = vec![
//         false, true, true, true, false, false,
//         false, false, false, false, false, false,
//         true, true, true, true, false, true,
//         true, true, true, true, true, true,
//     ];

//     // f, t, t, t, f, f
//     // f, f, f, f, f, f
//     // t, t, t, t, f, t
//     // t, t, t, t, t, t

//     let rectangles = break_into_rects(&grid, 6, 4);
//     for rect in rectangles {
//         println!("{:?}", rect)
//     }
// }


const MAX_ITERATIONS: usize = 10;
fn break_into_rects(grid: &Vec<bool>, width: u32, height: u32) -> Vec<(u32, u32, u32, u32)>{
    let mut working_grid = grid.clone();
    let mut rects: Vec<(u32, u32, u32, u32)> = Vec::new(); // x_pos, y_pos, width, height
    let w = width as usize;

    let mut iteration_count = 0;

    while iteration_count < MAX_ITERATIONS {
        let all_rects = get_all_rectangles(&working_grid, width, height);

        // prioritise rects with larger areas
        let mut sorted_rects = quicksort_descending(all_rects);
        let mut rect_count = sorted_rects.len();

        #[allow(unused_assignments)]
        let mut j = 0;
        for i in 0..rect_count {
            if i >= rect_count {break;}
            let rect_one = sorted_rects[i].1;
            j = i + 1;
            // remove overlapping meshes
            while j < rect_count {
                let rect_two = sorted_rects[j].1;
                if (rect_one.0 > rect_two.0 + rect_two.2) ||
                    (rect_one.0 + rect_one.2 < rect_two.0) ||
                    (rect_one.1 > rect_two.1 + rect_two.3) ||
                    (rect_one.1 + rect_one.3 < rect_two.1) {
                    j += 1;
                } else {
                    sorted_rects.remove(j);
                    rect_count -= 1;
                }
            }
            // change rect values to false in working grid
            for x in rect_one.0..(rect_one.0 + rect_one.2) {
                for y in rect_one.1..(rect_one.1 + rect_one.3) {
                    working_grid[y as usize * w + x as usize] = false;
                }
            }
        }
        for rect in sorted_rects {
            rects.push(rect.1)
        }
        // if there are not rectangles left to find, stop
        if !working_grid.contains(&true) {
            break;
        }
        iteration_count += 1;
    }

    // println!("{:?}", rects);
    rects

}

// https://orbi.uliege.be/bitstream/2268/23634/1/Vandroogenbroeck2011Object.pdf
// YOINK
fn get_all_rectangles(grid: &Vec<bool>, width: u32, height: u32) -> Vec<(u32, (u32, u32, u32, u32))> {
    let mut rectangles: Vec<(u32, (u32, u32, u32, u32))> = Vec::new(); // area, (x_pos, y_pos, width, height)
    let w = width as usize;
    let h = height as usize;

    let mut distance_to_north = vec![0; w * h];
    for x in 0..w {
        if grid[x] {
            distance_to_north[x] = 0;
        } else {
            distance_to_north[x] = -1;
        }
    }
    for y in 1..h {
        for x in 0..w {
            if !grid[y * w + x] {
                distance_to_north[y * w + x] = -1;
            } else {
                distance_to_north[y * w + x] = distance_to_north[(y-1) * w + x] + 1;
            }
        }
    }

    let mut distance_to_south = vec![0; w * h];
    for x in 0..w {
        if grid[(h-1) * w + x] {
            distance_to_south[(h-1) * w + x] = 0;
        } else {
            distance_to_south[(h-1) * w + x] = -1;
        }
    }
    for y in (0..h-1).rev() {
        for x in 0..w {
            if !grid[y * w + x] {
                distance_to_south[y * w + x] = -1;
            } else {
                distance_to_south[y * w + x] = distance_to_south[(y+1) * w + x] + 1;
            }
        }
    }

    for x in (0..w).rev() {
        let mut max_south = h as i32;
        for y in (0..h).rev() {
            max_south += 1;
            if grid[y * w + x] && (x == 0 || !grid[y * w + x - 1]) {
                let mut north = distance_to_north[y * w + x];
                let mut south = distance_to_south[y * w + x];
                let mut width = 1;
                while x + width < w && grid[y * w + x + width] {
                    let next_north = distance_to_north[y * w + x + width];
                    let next_south = distance_to_south[y * w + x + width];
                    if (next_north < north) | (next_south < south) {
                        if south < max_south {
                            let area = width as u32 * (north as u32 + south as u32 + 1);
                            rectangles.push((area, (x as u32, (y as i32 - north) as u32, width as u32, (north + south + 1) as u32)));
                        }
                        if next_north < north {
                            north = next_north;
                        }
                        if next_south < south {
                            south = next_south;
                        }
                    }
                    width += 1;
                }
                if south < max_south {
                    let area = width as u32 * (north as u32 + south as u32 + 1);
                    rectangles.push((area, (x as u32, (y as i32 - north) as u32, width as u32, (north + south + 1) as u32)));
                }
                max_south = 0;
            }
        }
    }

    rectangles
}