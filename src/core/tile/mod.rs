use macroquad::math::Vec2;
use crate::{DrawBatch, Object, World};
use std::any::Any;
use serde::{Serialize, Deserialize};
use crate::core::save::Vec2Save;
use std::collections::HashMap;

/// Represents a static game element that is part of the world's terrain or environment.
/// Tiles are the basic building blocks of the game world and are typically used for terrain.
pub trait Tile: Any + Send + Sync {
    /// Returns a unique identifier for the tile type
    fn get_type_tag(&self) -> &'static str;
    
    /// Returns the position of the tile in world coordinates
    fn get_pos(&self) -> Vec2;
    
    /// Returns the size of the tile in world units
    fn get_size(&self) -> Vec2;

    /// Called every frame to update the tile's state
    /// 
    /// - `dt`: Time elapsed since the last frame in seconds
    /// - `world`: Reference to the game world for interaction
    fn tick(&mut self, _dt: f32, _world: &mut World) {}
    
    /// Draws the tile on the screen
    /// 
    /// - `batch`: The draw batch to add drawing commands to
    /// - `pos`: The position to draw the tile at
    fn draw(&self, batch: &mut DrawBatch, pos: Vec2);

    /// Sets the position of the tile in world coordinates
    fn set_pos(&mut self, pos: Vec2);
    
    /// Sets the size of the tile in world units
    fn set_size(&mut self, _size: Vec2) {}

    /// Called when object right-clicks on this tile.  
    /// 
    /// - `obj`: The object that initiated the right-click.
    fn on_right_interact(&mut self, _obj: &mut dyn Object) { }  

    /// Called when object left-clicks on this tile.  
    /// 
    /// - `obj`: The object that initiated the left-click.
    fn on_left_interact(&mut self, _obj: &mut dyn Object) { }  

    /// Creates a boxed clone of this tile
    fn clone_box(&self) -> Box<dyn Tile>;
}

/// Serializable data structure representing a tile's state.
/// Used for saving and loading tile states from disk.
#[derive(Serialize, Deserialize)]
pub struct TileData {
    /// Unique identifier of the tile's type
    pub type_tag: String,
    /// Position of the tile in world coordinates
    pub pos: Vec2Save,
    /// Size of the tile in world units
    pub size: Vec2Save,
}

/// Manages the registration and instantiation of tile types.
/// Maintains a collection of tile prototypes that can be cloned to create new instances.
pub struct TileRegistry {
    /// Map of tile type tags to their prototype instances
    prototypes: HashMap<String, Box<dyn Tile>>,
}

impl Default for TileRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl TileRegistry {
    /// Creates a new, empty TileRegistry
    pub fn new() -> Self {
        Self {
            prototypes: HashMap::new(),
        }
    }

    /// Registers a new tile type with the registry
    /// 
    /// - `tile`: The prototype tile to register
    /// - `T`: Type parameter that implements both Tile and 'static
    pub fn register<T: Tile + 'static>(&mut self, tile: T) {
        self.prototypes.insert(tile.get_type_tag().to_string(), Box::new(tile));
    }

    /// Creates a new instance of a tile by its type tag
    /// 
    /// - `type_tag`: The type identifier of the tile to create
    /// 
    /// Returns `Some(boxed_tile)` if found, `None` otherwise
    pub fn create_tile_by_id(&self, type_tag: &str) -> Option<Box<dyn Tile>> {
        self.prototypes.get(type_tag).map(|proto| proto.clone_box())
    }

    /// Deserializes a tile from a JSON string
    /// 
    /// - `data`: JSON string containing serialized tile data
    /// 
    /// Returns a boxed tile on success, or an error message on failure
    pub fn deserialize_tile(&self, data: &str) -> Result<Box<dyn Tile>, String> {
        let data: TileData = serde_json::from_str(data)
            .map_err(|e| format!("Failed to deserialize TileData: {}", e))?;

        let prototype = self.prototypes.get(&data.type_tag)
            .ok_or_else(|| format!("Unknown tile type: {}", data.type_tag))?;

        let mut tile = prototype.clone_box();
        tile.set_pos(Vec2::from(data.pos));
        tile.set_size(Vec2::from(data.size));

        Ok(tile)
    }
}

/// Trait for tiles that can be serialized to and from strings.
/// Primarily used for saving and loading game states.
pub trait SerializableTile {
    /// Serializes the tile to a JSON string
    fn serialize(&self) -> String;
}

// Default implementation of SerializableTile for any type implementing Tile
impl SerializableTile for dyn Tile {
    /// Serializes the tile's data to a JSON string
    /// Includes type tag, position, and size information
    fn serialize(&self) -> String {
        let data = TileData {
            type_tag: self.get_type_tag().to_string(),
            pos: Vec2Save::from(self.get_pos()),
            size: Vec2Save::from(self.get_size()),
        };
        serde_json::to_string(&data).unwrap()
    }
}
