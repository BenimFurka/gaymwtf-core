use macroquad::math::{vec2, Vec2};
use serde::{Deserialize, Serialize};

use crate::{
    core::save::vec2::Vec2Save,
    Object, ObjectRegistry, SerializableObject, SerializableTile, Tile, TileRegistry, World,
    log_chunk,
    DrawBatch, CHUNK_PIXELS, CHUNK_SIZE, TILE_SIZE, OBJECT_ACTIVATION_MARGIN,
};

pub struct Chunk {
    pub tiles: Vec<Box<dyn Tile>>,
    pub objects: Vec<Box<dyn Object>>,
    pub pos: Vec2,
    bounds: (Vec2, Vec2),
    visible_tiles: Vec<usize>,
    active_objects: Vec<usize>,
}

#[derive(Serialize, Deserialize)]
pub struct ChunkData {
    pub pos: Vec2Save,
    pub tiles: Vec<String>,
    pub objects: Vec<String>,
}

impl Chunk {
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

    pub fn draw_objects(&mut self, batch: &mut DrawBatch) {
        for &obj_index in &self.active_objects {
            if let Some(obj) = self.objects.get(obj_index) {
                obj.draw(batch);
            }
        }
    }

    pub fn is_visible(&self, camera_pos: Vec2, screen_size: Vec2) -> bool {
        let screen_min = camera_pos - screen_size / 2.0;
        let screen_max = camera_pos + screen_size / 2.0;

        !(self.bounds.1.x < screen_min.x
            || self.bounds.0.x > screen_max.x
            || self.bounds.1.y < screen_min.y
            || self.bounds.0.y > screen_max.y)
    }

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

    pub fn get_objects_by_type(&self, type_tag: &str) -> Vec<&Box<dyn Object>> {
        let mut objects = Vec::new();

        for obj in &self.objects {
            if obj.get_type_tag() == type_tag {
                objects.push(obj);
            }
        }
        objects
    }

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
