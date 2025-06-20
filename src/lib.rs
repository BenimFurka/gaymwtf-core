pub mod core;
pub mod engine;
pub mod utils;

pub use crate::core::world::world::{World, WorldData};
pub use crate::core::chunk::chunk::{Chunk, ChunkData};
pub use crate::core::tile::tile::{Tile, TileRegistry, SerializableTile, TileData};
pub use crate::core::object::object::{Direction, Object, ObjectRegistry, SerializableObject, ObjectData};
pub use crate::core::biome::biome::{Biome, BiomeRegistry};
pub use crate::core::save::vec2::Vec2Save;
pub use crate::core::menu::menu::{Menu, MenuAction};

pub use crate::engine::texture::{load_file_sync, load_texture_sync};

pub use crate::utils::draw::DrawBatch;
pub use crate::utils::logger::GameLogger;

pub use crate::utils::settings::{TILE_SIZE, CHUNK_SIZE, CHUNK_PIXELS, OBJECT_ACTIVATION_MARGIN};

