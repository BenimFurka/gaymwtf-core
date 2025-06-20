use macroquad::prelude::*;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use std::fs;

use crate::{
    Chunk, ObjectRegistry, TileRegistry, BiomeRegistry,
    DrawBatch, CHUNK_PIXELS, log_world, Tile, Object
};

#[derive(Serialize, Deserialize)]
pub struct WorldData {
    pub name: String,
}

pub struct World {
    pub chunks: HashMap<(i32, i32), Chunk>,
    pub tile_registry: TileRegistry,
    pub object_registry: ObjectRegistry,
    pub biome_registry: BiomeRegistry,
    visible_chunks: Vec<(i32, i32)>,
    draw_batch: DrawBatch,
    world_name: String,
}

impl World {
    pub fn new(world_name: &str, tile_registry: TileRegistry, object_registry: ObjectRegistry, biome_registry: BiomeRegistry) -> Self {
        log_world!(log::Level::Info, "Creating world '{}'", world_name);
        Self {
            chunks: HashMap::new(),
            tile_registry,
            object_registry,
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

    pub fn load_world(save_dir: &str, tile_registry: TileRegistry, object_registry: ObjectRegistry, biome_registry: BiomeRegistry) -> Result<Self, String> {
        let world_data_path = format!("{}/world.json", save_dir);
        let data = fs::read_to_string(world_data_path).map_err(|e| e.to_string())?;
        let world_data: WorldData = serde_json::from_str(&data).map_err(|e| e.to_string())?;

        let mut world = Self::new(&world_data.name, tile_registry, object_registry, biome_registry);

        let chunks_dir = format!("{}/chunks", save_dir);
        if let Ok(entries) = fs::read_dir(chunks_dir) {
            for entry in entries.flatten() {
                if let Ok(chunk_data) = fs::read_to_string(entry.path()) {
                    if let Ok(chunk) = Chunk::deserialize(&chunk_data, &world.tile_registry, &world.object_registry) {
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
                for (obj_index, obj) in chunk.objects.iter().enumerate() {
                    let new_chunk_pos = self.get_chunk_coords(obj.get_pos());
                    if new_chunk_pos != chunk_pos {
                        movements.push((chunk_pos, new_chunk_pos, obj_index));
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

        for (old_pos, new_pos, obj_index) in movements {
            if let Some(mut chunk) = self.chunks.remove(&old_pos) {
                if obj_index < chunk.objects.len() {
                    let obj = chunk.objects.remove(obj_index);
                    self.chunks.insert(old_pos, chunk);
                    if let Some(new_chunk) = self.chunks.get_mut(&new_pos) {
                        new_chunk.objects.push(obj);
                    }
                } else {
                    self.chunks.insert(old_pos, chunk);
                }
            }
        }

        self.check_obj_collisions();

        let visible_chunks_copy = self.visible_chunks.clone();
        for chunk_pos in visible_chunks_copy {
            if let Some(mut chunk) = self.chunks.remove(&chunk_pos) {
                chunk.update(self, camera_pos, screen_size, get_frame_time());
                self.chunks.insert(chunk_pos, chunk);
            }
        }
    }
    fn check_obj_collisions(&mut self) {
        let mut objects: Vec<Box<dyn Object>> = Vec::new();
        let mut chunk_positions = Vec::new();

        for &chunk_pos in &self.visible_chunks {
            if let Some(chunk) = self.chunks.get_mut(&chunk_pos) {
                for obj in chunk.objects.drain(..) {
                    objects.push(obj);
                    chunk_positions.push(chunk_pos);
                }
            }
        }

        for i in 0..objects.len() {
            for j in (i + 1)..objects.len() {
                let (obj1, obj2) = objects.split_at_mut(j);
                let obj1 = &mut obj1[i];
                let obj2 = &mut obj2[0];

                let pos1 = obj1.get_pos();
                let velocity1 = obj1.get_velocity();
                let size1 = obj1.get_size();
                let next_pos1 = pos1 + velocity1;

                let pos2 = obj2.get_pos();
                let velocity2 = obj2.get_velocity();
                let size2 = obj2.get_size();
                let next_pos2 = pos2 + velocity2;

                let will_collide = next_pos1.x < next_pos2.x + size2.x &&
                                 next_pos1.x + size1.x > next_pos2.x &&
                                 next_pos1.y < next_pos2.y + size2.y &&
                                 next_pos1.y + size1.y > next_pos2.y;

                let moving_towards_each_other = {
                    let relative_velocity = velocity1 - velocity2;
                    let direction = pos2 - pos1;
                    relative_velocity.dot(direction) > 0.0
                };

                if will_collide && moving_towards_each_other {
                    let obj1: &mut dyn Object = &mut **obj1;
                    let obj2: &mut dyn Object = &mut **obj2;
                    
                    obj1.collision(obj2);
                    obj2.collision(obj1);
                }
            }
        }

        for (obj, &chunk_pos) in objects.into_iter().zip(chunk_positions.iter()) {
            if let Some(chunk) = self.chunks.get_mut(&chunk_pos) {
                chunk.objects.push(obj);
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
                chunk.draw_objects(&mut self.draw_batch);
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

    pub fn get_objects_by_type(&self, type_tag: &str) -> Vec<&Box<dyn Object>> {
        let mut objects = Vec::new();
        for &chunk_pos in &self.visible_chunks {
            if let Some(chunk) = self.chunks.get(&chunk_pos) {
                for obj in &chunk.objects {
                    if obj.get_type_tag() == type_tag {
                        objects.push(obj);
                    }
                }
            }
        }
        objects
    }

    pub fn get_tiles_by_type(&self, type_tag: &str) -> Vec<&Box<dyn Tile>> {
        let mut tiles = Vec::new();

        for &chunk_pos in &self.visible_chunks {
            if let Some(chunk) = self.chunks.get(&chunk_pos) {
                for tile in &chunk.tiles {
                    if tile.get_type_tag() == type_tag {
                        tiles.push(tile);
                    }
                }
            }
        }
        tiles
    }
}
