use macroquad::math::Vec2;
use crate::{Chunk, DrawBatch, Entity, World};
use std::any::Any;
use serde::{Serialize, Deserialize};
use crate::core::save::vec2::Vec2Save;
use std::collections::HashMap;

pub trait Tile: Any + Send + Sync {
    fn get_type_tag(&self) -> &'static str;
    fn get_pos(&self) -> Vec2;
    fn set_pos(&mut self, pos: Vec2);
    fn get_size(&self) -> Vec2;
    fn clone_box(&self) -> Box<dyn Tile>;

    fn draw(&self, batch: &mut DrawBatch, pos: Vec2);
    fn may_pass(&self) -> bool;

    fn interact(&mut self, _other: &mut dyn Entity) -> bool {
        false
    }

    fn update(&mut self, _dt: f32, _world: &mut World) {}

    fn tick(&mut self, _level: &mut Chunk) {}

    fn set_size(&mut self, _size: Vec2) {}
}

#[derive(Serialize, Deserialize)]
pub struct TileData {
    pub type_tag: String,
    pub pos: Vec2Save,
    pub size: Vec2Save,
}

pub struct TileRegistry {
    prototypes: HashMap<String, Box<dyn Tile>>,
}

impl Default for TileRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl TileRegistry {
    pub fn new() -> Self {
        Self {
            prototypes: HashMap::new(),
        }
    }

    pub fn register<T: Tile + 'static>(&mut self, tile: T) {
        self.prototypes.insert(tile.get_type_tag().to_string(), Box::new(tile));
    }

    pub fn create_tile_by_id(&self, type_tag: &str) -> Option<Box<dyn Tile>> {
        self.prototypes.get(type_tag).map(|proto| proto.clone_box())
    }

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

pub trait SerializableTile {
    fn serialize(&self) -> String;
}

impl SerializableTile for dyn Tile {
    fn serialize(&self) -> String {
        let data = TileData {
            type_tag: self.get_type_tag().to_string(),
            pos: Vec2Save::from(self.get_pos()),
            size: Vec2Save::from(self.get_size()),
        };
        serde_json::to_string(&data).unwrap()
    }
}
