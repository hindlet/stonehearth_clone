use std::time::{Duration, Instant};
use bevy::{prelude::*, tasks::{block_on, futures_lite::future, AsyncComputeTaskPool, Task}};
use super::{VoxelMesher, VoxelGrid, mesh_voxels};


pub struct MeshUpgradePlugin;

impl Plugin for MeshUpgradePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(DefaultMeshUpgradeTime(Duration::from_secs(60)))
            .add_systems(Update, (check_for_mesh_upgrades, apply_mesh_upgrades));
    }
}


/// naive default will change to fancy mesh after 1 minute
/// naive custom will after a set time
#[derive(Component)]
pub enum MeshUpgradeStatus {
    NaiveDefault(Instant),
    NaiveCustom(Instant, Duration),
    Fancy
}

#[derive(Component)]
struct MeshUpgradeTask(Task<Mesh>);

#[derive(Resource)]
struct DefaultMeshUpgradeTime(Duration);


fn check_for_mesh_upgrades(
    mut commands: Commands,
    default_upgrade_time: Res<DefaultMeshUpgradeTime>,
    mesh_query: Query<(Entity, &VoxelGrid, &MeshUpgradeStatus), Without<MeshUpgradeTask>>
) {
    for (id, grid, mesh_status) in mesh_query.iter() {
        match mesh_status {
            MeshUpgradeStatus::NaiveDefault(start_time) => {
                // println!("{}", start_time.elapsed().as_secs_f32());
                if start_time.elapsed() >= default_upgrade_time.0 {
                    // println!("WOOOO TASK TIME");
                    spawn_mesh_upgrade_task(&mut commands, id, grid.clone());
                    break;
                }
            },
            MeshUpgradeStatus::NaiveCustom(start_time, duration) => {
                if start_time.elapsed() >= *duration {
                    spawn_mesh_upgrade_task(&mut commands, id, grid.clone());
                    break;
                }
            }
            _ => {}
        }
    }
}

fn spawn_mesh_upgrade_task(
    commands: &mut Commands,
    chunk_id: Entity,
    chunk_grid: VoxelGrid
){
    let thread_pool = AsyncComputeTaskPool::get();

    let task = thread_pool.spawn(async move {
        mesh_voxels(&chunk_grid, VoxelMesher::FancyMeshing)
    });

    commands.entity(chunk_id).insert(MeshUpgradeTask(task));
}


fn apply_mesh_upgrades(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    default_upgrade_time: Res<DefaultMeshUpgradeTime>,

    mut upgrade_targets: Query<(&mut Handle<Mesh>, &mut MeshUpgradeStatus)>,
    mut tasks: Query<(Entity, &mut MeshUpgradeTask)>,
) {
    for (task_target, mut task) in &mut tasks {
        if let Some(mesh) = block_on(future::poll_once(&mut task.0)) {
            if let Ok((mut mesh_handle, mut mesh_status)) = upgrade_targets.get_mut(task_target) {

                match *mesh_status {
                    MeshUpgradeStatus::NaiveDefault(start_time) => {
                        if start_time.elapsed() >= default_upgrade_time.0 { // if this is false the mesh has been edited since the task was dispatched
                            *mesh_handle = meshes.add(mesh);
                            *mesh_status = MeshUpgradeStatus::Fancy;
                        }
                    },
                    MeshUpgradeStatus::NaiveCustom(start_time, duration) => {
                        if start_time.elapsed() >= duration { // if this is false the mesh has been edited since the task was dispatched
                            *mesh_handle = meshes.add(mesh);
                            *mesh_status = MeshUpgradeStatus::Fancy;
                        }
                    },
                    _ => {println!("OH NO OH DEAR FUCK")}
                }
            } else {
                println!("OH NO OH DEAR FUCK")
            }
            commands.entity(task_target).remove::<MeshUpgradeTask>();
        }
    }
}
