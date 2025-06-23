use macroquad::math::{vec2, Vec2};
use serde::{Deserialize, Serialize};

use crate::{
    core::save::Vec2Save,
    Object, ObjectRegistry, SerializableObject, SerializableTile, Tile, TileRegistry, World,
    log_chunk,
    DrawBatch, CHUNK_PIXELS, CHUNK_SIZE, TILE_SIZE, OBJECT_ACTIVATION_MARGIN,
};

/// A fixed-size segment of the game world that contains tiles and objects.
/// Chunks are used to efficiently manage and render the game world by dividing it into smaller,
/// more manageable pieces. Each chunk contains its own set of visible tiles and active objects.
pub struct Chunk {
    /// Collection of all tiles in this chunk
    pub tiles: Vec<Box<dyn Tile>>,
    /// Collection of all objects currently in this chunk
    pub objects: Vec<Box<dyn Object>>,
    /// Position of this chunk in chunk coordinates (not world coordinates)
    pub pos: Vec2,
    /// Bounding box of this chunk in world coordinates
    bounds: (Vec2, Vec2),
    /// Indices of tiles that are currently visible on screen
    visible_tiles: Vec<usize>,
    /// Indices of objects that are currently active (in or near the viewport)
    active_objects: Vec<usize>,
}

/// Serializable data structure representing a chunk's state.
/// Used for saving and loading chunk data from disk.
#[derive(Serialize, Deserialize)]
pub struct ChunkData {
    /// Position of the chunk in chunk coordinates
    pub pos: Vec2Save,
    /// Serialized data of all tiles in this chunk
    pub tiles: Vec<String>,
    /// Serialized data of all objects in this chunk
    pub objects: Vec<String>,
}

impl Chunk {
    /// Creates a new, empty chunk at the specified position
    /// 
    /// - `pos`: The position of the chunk in chunk coordinates
    /// 
    pub fn new(pos: Vec2) -> Self {
        log_chunk!(log::Level::Debug, "Creating new chunk at {:?}", pos);
        let min = pos * CHUNK_PIXELS;
        let max = min + vec2(CHUNK_PIXELS, CHUNK_PIXELS);

        Self {
            tiles: Vec::with_capacity(CHUNK_SIZE * CHUNK_SIZE),
            objects: Vec::new(),
            pos,
            bounds: (min, max),
            visible_tiles: Vec::new(),
            active_objects: Vec::new(),
        }
    }

    /// Updates the chunk's state
    /// 
    /// - `world`: Reference to the game world
    /// - `camera_pos`: Current camera position in world coordinates
    /// - `screen_size`: Size of the game window
    /// - `dt`: Time elapsed since the last frame in seconds
    pub fn update(&mut self, world: &mut World, camera_pos: Vec2, screen_size: Vec2, dt: f32) {
        if !self.is_visible(camera_pos, screen_size) {
            return;
        }

        self.update_active_objects(camera_pos, screen_size);
        self.update_visible_tiles(camera_pos, screen_size);

        for &obj_index in &self.active_objects {
            if let Some(obj) = self.objects.get_mut(obj_index) {
                obj.tick(dt, world);
            }
        }

        for &tile_index in &self.visible_tiles {
            if let Some(tile) = self.tiles.get_mut(tile_index) {
                tile.tick(dt, world);
            }
        }
    }

    /// Draws all visible tiles in this chunk
    /// 
    /// - `camera_pos`: Current camera position in world coordinates
    /// - `screen_size`: Size of the game window
    /// - `batch`: The draw batch to add drawing commands to
    pub fn draw_tiles(&mut self, camera_pos: Vec2, screen_size: Vec2, batch: &mut DrawBatch) {
        if !self.is_visible(camera_pos, screen_size) {
            return;
        }

        self.update_visible_tiles(camera_pos, screen_size);

        for &tile_index in &self.visible_tiles {
            let tile = &self.tiles[tile_index];
            tile.draw(batch, tile.get_pos());
        }
    }

    /// Draws all active objects in this chunk
    /// 
    /// - `batch`: The draw batch to add drawing commands to
    pub fn draw_objects(&mut self, batch: &mut DrawBatch) {
        for &obj_index in &self.active_objects {
            if let Some(obj) = self.objects.get(obj_index) {
                obj.draw(batch);
            }
        }
    }

    /// Checks if this chunk is currently visible on screen
    /// 
    /// - `camera_pos`: Current camera position in world coordinates
    /// - `screen_size`: Size of the game window
    /// 
    /// Returns `true` if any part of this chunk is visible on screen
    pub fn is_visible(&self, camera_pos: Vec2, screen_size: Vec2) -> bool {
        let screen_min = camera_pos - screen_size / 2.0;
        let screen_max = camera_pos + screen_size / 2.0;

        !(self.bounds.1.x < screen_min.x
            || self.bounds.0.x > screen_max.x
            || self.bounds.1.y < screen_min.y
            || self.bounds.0.y > screen_max.y)
    }

    /// Updates the list of tiles that are currently visible on screen
    /// 
    /// - `camera_pos`: Current camera position in world coordinates
    /// - `screen_size`: Size of the game window
    fn update_visible_tiles(&mut self, camera_pos: Vec2, screen_size: Vec2) {
        self.visible_tiles.clear();
        let screen_min = camera_pos - screen_size / 2.0;
        let screen_max = camera_pos + screen_size / 2.0;

        let start_x = ((screen_min.x - self.bounds.0.x) / TILE_SIZE).floor() as i32;
        let end_x = ((screen_max.x - self.bounds.0.x) / TILE_SIZE).ceil() as i32;
        let start_y = ((screen_min.y - self.bounds.0.y) / TILE_SIZE).floor() as i32;
        let end_y = ((screen_max.y - self.bounds.0.y) / TILE_SIZE).ceil() as i32;

        let start_x = start_x.max(0).min(CHUNK_SIZE as i32 - 1) as usize;
        let end_x = end_x.max(0).min(CHUNK_SIZE as i32) as usize;
        let start_y = start_y.max(0).min(CHUNK_SIZE as i32 - 1) as usize;
        let end_y = end_y.max(0).min(CHUNK_SIZE as i32) as usize;

        for y in start_y..end_y {
            for x in start_x..end_x {
                let index = y * CHUNK_SIZE + x;
                if index < self.tiles.len() {
                    self.visible_tiles.push(index);
                }
            }
        }
    }

    /// Updates the list of objects that are currently active (in or near the viewport)
    /// 
    /// - `camera_pos`: Current camera position in world coordinates
    /// - `screen_size`: Size of the game window
    fn update_active_objects(&mut self, camera_pos: Vec2, screen_size: Vec2) {
        self.active_objects.clear();
        let screen_min = camera_pos - screen_size / 2.0 - Vec2::splat(OBJECT_ACTIVATION_MARGIN);
        let screen_max = camera_pos + screen_size / 2.0 + Vec2::splat(OBJECT_ACTIVATION_MARGIN);

        for (index, obj) in self.objects.iter().enumerate() {
            let pos = obj.get_pos();
            if pos.x >= screen_min.x && pos.x <= screen_max.x && pos.y >= screen_min.y && pos.y <= screen_max.y {
                self.active_objects.push(index);
            }
        }
    }

    /// Serializes this chunk into a string
    /// Returns a JSON string containing the chunk's data
    pub fn serialize(&self) -> String {
        let tiles: Vec<String> = self.tiles.iter().map(|tile| tile.serialize()).collect();
        let objects: Vec<String> = self.objects.iter().map(|obj| obj.serialize()).collect();
        let data = ChunkData {
            pos: Vec2Save::from(self.pos),
            tiles,
            objects,
        };
        serde_json::to_string(&data).unwrap()
    }

    /// Deserializes a chunk from a string
    /// 
    /// - `data`: The serialized chunk data
    /// - `tile_registry`: Registry containing tile prototypes
    /// - `object_registry`: Registry containing object prototypes
    /// 
    /// Returns a new Chunk instance or an error if deserialization fails
    pub fn deserialize(
        data: &str,
        tile_registry: &TileRegistry,
        object_registry: &ObjectRegistry,
    ) -> Result<Self, String> {
        let data: ChunkData = serde_json::from_str(data).map_err(|e| e.to_string())?;
        let pos = Vec2::from(data.pos);

        let tiles_res: Result<Vec<_>, _> = data.tiles.iter().map(|tile_data| tile_registry.deserialize_tile(tile_data)).collect();
        let objects_res: Result<Vec<_>, _> = data.objects.iter().map(|object_data| object_registry.deserialize_object(object_data)).collect();

        let mut chunk = Chunk::new(pos);
        chunk.tiles = tiles_res?;
        chunk.objects = objects_res?;

        Ok(chunk)
    }

    /// Returns all objects of the specified type in this chunk
    /// 
    /// - `type_tag`: The type of objects to find
    /// 
    /// Returns a vector of references to matching objects
    pub fn get_objects_by_type(&self, type_tag: &str) -> Vec<&Box<dyn Object>> {
        let mut objects = Vec::new();

        for obj in &self.objects {
            if obj.get_type_tag() == type_tag {
                objects.push(obj);
            }
        }
        objects
    }

    /// Returns all tiles of the specified type in this chunk
    /// 
    /// - `type_tag`: The type of tiles to find
    /// 
    /// Returns a vector of references to matching tiles
    pub fn get_tiles_by_type(&self, type_tag: &str) -> Vec<&Box<dyn Tile>> {
        let mut tiles = Vec::new();

        for tile in &self.tiles {
            if tile.get_type_tag() == type_tag {
                tiles.push(tile);
            }
        }
        tiles
    }
}
