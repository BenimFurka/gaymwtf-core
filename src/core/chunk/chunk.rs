use macroquad::math::{vec2, Vec2};
use serde::{Deserialize, Serialize};

use crate::{
    core::save::vec2::Vec2Save,
    Entity, EntityRegistry, SerializableEntity, SerializableTile, Tile, TileRegistry, World,
    log_chunk,
    DrawBatch, CHUNK_PIXELS, CHUNK_SIZE, TILE_SIZE, ENTITY_ACTIVATION_MARGIN,
};

pub struct Chunk {
    pub tiles: Vec<Box<dyn Tile>>,
    pub entities: Vec<Box<dyn Entity>>,
    pub pos: Vec2,
    bounds: (Vec2, Vec2),
    visible_tiles: Vec<usize>,
    active_entities: Vec<usize>,
}

#[derive(Serialize, Deserialize)]
pub struct ChunkData {
    pub pos: Vec2Save,
    pub tiles: Vec<String>,
    pub entities: Vec<String>,
}

impl Chunk {
    pub fn new(pos: Vec2) -> Self {
        log_chunk!(log::Level::Debug, "Creating new chunk at {:?}", pos);
        let min = pos * CHUNK_PIXELS;
        let max = min + vec2(CHUNK_PIXELS, CHUNK_PIXELS);

        Self {
            tiles: Vec::with_capacity(CHUNK_SIZE * CHUNK_SIZE),
            entities: Vec::new(),
            pos,
            bounds: (min, max),
            visible_tiles: Vec::new(),
            active_entities: Vec::new(),
        }
    }

    pub fn update(&mut self, world: &mut World, camera_pos: Vec2, screen_size: Vec2, dt: f32) {
        if !self.is_visible(camera_pos, screen_size) {
            return;
        }

        self.update_active_entities(camera_pos, screen_size);
        self.update_visible_tiles(camera_pos, screen_size);

        for &entity_index in &self.active_entities {
            if let Some(entity) = self.entities.get_mut(entity_index) {
                entity.tick(dt, world);
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

    pub fn draw_entities(&mut self, batch: &mut DrawBatch) {
        for &entity_index in &self.active_entities {
            if let Some(entity) = self.entities.get(entity_index) {
                entity.draw(batch);
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

    fn update_active_entities(&mut self, camera_pos: Vec2, screen_size: Vec2) {
        self.active_entities.clear();
        let screen_min = camera_pos - screen_size / 2.0 - Vec2::splat(ENTITY_ACTIVATION_MARGIN);
        let screen_max = camera_pos + screen_size / 2.0 + Vec2::splat(ENTITY_ACTIVATION_MARGIN);

        for (index, entity) in self.entities.iter().enumerate() {
            let pos = entity.get_pos();
            if pos.x >= screen_min.x && pos.x <= screen_max.x && pos.y >= screen_min.y && pos.y <= screen_max.y {
                self.active_entities.push(index);
            }
        }
    }

    pub fn serialize(&self) -> String {
        let tiles: Vec<String> = self.tiles.iter().map(|tile| tile.serialize()).collect();
        let entities: Vec<String> = self.entities.iter().map(|entity| entity.serialize()).collect();
        let data = ChunkData {
            pos: Vec2Save::from(self.pos),
            tiles,
            entities,
        };
        serde_json::to_string(&data).unwrap()
    }

    pub fn deserialize(
        data: &str,
        tile_registry: &TileRegistry,
        entity_registry: &EntityRegistry,
    ) -> Result<Self, String> {
        let data: ChunkData = serde_json::from_str(data).map_err(|e| e.to_string())?;
        let pos = Vec2::from(data.pos);

        let tiles_res: Result<Vec<_>, _> = data.tiles.iter().map(|tile_data| tile_registry.deserialize_tile(tile_data)).collect();
        let entities_res: Result<Vec<_>, _> = data.entities.iter().map(|entity_data| entity_registry.deserialize_entity(entity_data)).collect();

        let mut chunk = Chunk::new(pos);
        chunk.tiles = tiles_res?;
        chunk.entities = entities_res?;

        Ok(chunk)
    }
}
