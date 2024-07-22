use bevy::app::Plugin;

mod chunks;
mod terrain_gen;
pub use chunks::TerrainChunkBundle;
pub use terrain_gen::spawn_world_chunks;