pub mod core;
pub mod engine;
pub mod utils;

pub use crate::core::world::{World, WorldData};
pub use crate::core::chunk::{Chunk, ChunkData};
pub use crate::core::tile::{Tile, TileData, TileRegistry, SerializableTile};
pub use crate::core::object::{Object, ObjectData, ObjectRegistry, SerializableObject, Direction};
pub use crate::core::biome::{Biome, BiomeRegistry};
pub use crate::core::save::{Vec2Save};
pub use crate::core::ui::{Button, Label, MenuAction, Menu, Element, ButtonState};

pub use crate::engine::texture::{load_file_sync, load_texture_sync};

pub use crate::utils::draw::DrawBatch;
pub use crate::utils::logger::GameLogger;

pub use crate::utils::settings::{TILE_SIZE, CHUNK_SIZE, CHUNK_PIXELS, OBJECT_ACTIVATION_MARGIN};

