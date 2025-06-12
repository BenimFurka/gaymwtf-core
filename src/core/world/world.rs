use macroquad::prelude::*;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use std::fs;
use crate::Entity;

use crate::{
    Chunk, EntityRegistry, TileRegistry, BiomeRegistry,
    DrawBatch, CHUNK_PIXELS, log_world,
};

#[derive(Serialize, Deserialize)]
pub struct WorldData {
    pub name: String,
}

pub struct World {
    pub chunks: HashMap<(i32, i32), Chunk>,
    pub tile_registry: TileRegistry,
    pub entity_registry: EntityRegistry,
    pub biome_registry: BiomeRegistry,
    visible_chunks: Vec<(i32, i32)>,
    draw_batch: DrawBatch,
    world_name: String,
}

impl World {
    pub fn new(world_name: &str, tile_registry: TileRegistry, entity_registry: EntityRegistry, biome_registry: BiomeRegistry) -> Self {
        log_world!(log::Level::Info, "Creating world '{}'", world_name);
        Self {
            chunks: HashMap::new(),
            tile_registry,
            entity_registry,
            biome_registry,
            visible_chunks: Vec::new(),
            draw_batch: DrawBatch::new(),
            world_name: world_name.to_string(),
        }
    }

    pub fn add_chunk(&mut self, chunk: Chunk) {
        let chunk_key = (chunk.pos.x as i32, chunk.pos.y as i32);
        if !self.chunks.contains_key(&chunk_key) {
            self.chunks.insert(chunk_key, chunk);
        }
    }

    pub fn save_world(&self, save_dir: &str) -> Result<(), String> {
        let chunks_dir = format!("{}/chunks", save_dir);
        fs::create_dir_all(&chunks_dir).map_err(|e| e.to_string())?;

        let world_data = WorldData { name: self.world_name.clone() };
        let serialized = serde_json::to_string(&world_data).map_err(|e| e.to_string())?;
        fs::write(format!("{}/world.json", save_dir), serialized).map_err(|e| e.to_string())?;

        for (&(x, y), chunk) in &self.chunks {
            let chunk_path = format!("{}/chunk_{}_{}.json", chunks_dir, x, y);
            fs::write(chunk_path, chunk.serialize()).map_err(|e| e.to_string())?;
        }
        Ok(())
    }

    pub fn load_world(save_dir: &str, tile_registry: TileRegistry, entity_registry: EntityRegistry, biome_registry: BiomeRegistry) -> Result<Self, String> {
        let world_data_path = format!("{}/world.json", save_dir);
        let data = fs::read_to_string(world_data_path).map_err(|e| e.to_string())?;
        let world_data: WorldData = serde_json::from_str(&data).map_err(|e| e.to_string())?;

        let mut world = Self::new(&world_data.name, tile_registry, entity_registry, biome_registry);

        let chunks_dir = format!("{}/chunks", save_dir);
        if let Ok(entries) = fs::read_dir(chunks_dir) {
            for entry in entries.flatten() {
                if let Ok(chunk_data) = fs::read_to_string(entry.path()) {
                    if let Ok(chunk) = Chunk::deserialize(&chunk_data, &world.tile_registry, &world.entity_registry) {
                        world.add_chunk(chunk);
                    }
                }
            }
        }
        Ok(world)
    }

    pub fn update(&mut self, camera_pos: Vec2, screen_size: Vec2) {
        let current_chunk_coords = self.get_chunk_coords(camera_pos);
        self.update_visible_chunks(current_chunk_coords);

        let mut movements = Vec::new();
        for &chunk_pos in &self.visible_chunks {
            if let Some(chunk) = self.chunks.get(&chunk_pos) {
                for (entity_index, entity) in chunk.entities.iter().enumerate() {
                    let new_chunk_pos = self.get_chunk_coords(entity.get_pos());
                    if new_chunk_pos != chunk_pos {
                        movements.push((chunk_pos, new_chunk_pos, entity_index));
                    }
                }
            }
        }

        movements.sort_by(|a, b| {
            if a.0 == b.0 {
                b.2.cmp(&a.2)
            } else {
                std::cmp::Ordering::Equal
            }
        });

        for (old_pos, new_pos, entity_index) in movements {
            if let Some(mut chunk) = self.chunks.remove(&old_pos) {
                if entity_index < chunk.entities.len() {
                    let entity = chunk.entities.remove(entity_index);
                    self.chunks.insert(old_pos, chunk);
                    if let Some(new_chunk) = self.chunks.get_mut(&new_pos) {
                        new_chunk.entities.push(entity);
                    }
                } else {
                    self.chunks.insert(old_pos, chunk);
                }
            }
        }

        self.check_entity_collisions();

        let visible_chunks_copy = self.visible_chunks.clone();
        for chunk_pos in visible_chunks_copy {
            if let Some(mut chunk) = self.chunks.remove(&chunk_pos) {
                chunk.update(self, camera_pos, screen_size, get_frame_time());
                self.chunks.insert(chunk_pos, chunk);
            }
        }
    }
    fn check_entity_collisions(&mut self) {
        let mut entities: Vec<Box<dyn Entity>> = Vec::new();
        let mut chunk_positions = Vec::new();

        for &chunk_pos in &self.visible_chunks {
            if let Some(chunk) = self.chunks.get_mut(&chunk_pos) {
                for entity in chunk.entities.drain(..) {
                    entities.push(entity);
                    chunk_positions.push(chunk_pos);
                }
            }
        }

        for i in 0..entities.len() {
            for j in (i + 1)..entities.len() {
                let (entity1, entity2) = entities.split_at_mut(j);
                let entity1 = &mut entity1[i];
                let entity2 = &mut entity2[0];

                let pos1 = entity1.get_pos();
                let speed1 = entity1.get_speed();
                let size1 = entity1.get_size();
                let next_pos1 = pos1 + speed1;

                let pos2 = entity2.get_pos();
                let speed2 = entity2.get_speed();
                let size2 = entity2.get_size();
                let next_pos2 = pos2 + speed2;

                let will_collide = next_pos1.x < next_pos2.x + size2.x &&
                                 next_pos1.x + size1.x > next_pos2.x &&
                                 next_pos1.y < next_pos2.y + size2.y &&
                                 next_pos1.y + size1.y > next_pos2.y;

                let moving_towards_each_other = {
                    let relative_velocity = speed1 - speed2;
                    let direction = pos2 - pos1;
                    relative_velocity.dot(direction) > 0.0
                };

                if will_collide && moving_towards_each_other {
                    let mut entity1_copy = entity1.clone_box();
                    let mut entity2_copy = entity2.clone_box();
                    
                    entity1_copy.collision(&mut *entity2_copy);
                    entity2_copy.collision(&mut *entity1_copy);
                    
                    entity1.set_speed(entity1_copy.get_speed());
                    entity2.set_speed(entity2_copy.get_speed());
                }
            }
        }

        for (entity, &chunk_pos) in entities.into_iter().zip(chunk_positions.iter()) {
            if let Some(chunk) = self.chunks.get_mut(&chunk_pos) {
                chunk.entities.push(entity);
            }
        }
    }

    pub fn draw(&mut self, camera_pos: Vec2, screen_size: Vec2) {
        self.draw_batch.clear();
        for &chunk_pos in &self.visible_chunks {
            if let Some(chunk) = self.chunks.get_mut(&chunk_pos) {
                chunk.draw_tiles(camera_pos, screen_size, &mut self.draw_batch);
            }
        }
        self.draw_batch.draw();

        self.draw_batch.clear();
        for &chunk_pos in &self.visible_chunks {
            if let Some(chunk) = self.chunks.get_mut(&chunk_pos) {
                chunk.draw_entities(&mut self.draw_batch);
            }
        }
        self.draw_batch.draw();
    }

    fn update_visible_chunks(&mut self, camera_chunk: (i32, i32)) {
        self.visible_chunks.clear();
        let render_dist = 2;
        for y in -render_dist..=render_dist {
            for x in -render_dist..=render_dist {
                self.visible_chunks.push((camera_chunk.0 + x, camera_chunk.1 + y));
            }
        }
    }

    fn get_chunk_coords(&self, pos: Vec2) -> (i32, i32) {
        (
            (pos.x / CHUNK_PIXELS).floor() as i32,
            (pos.y / CHUNK_PIXELS).floor() as i32,
        )
    }
}
