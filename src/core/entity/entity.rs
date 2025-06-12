use std::any::Any;
use macroquad::math::Vec2;
use crate::utils::draw::DrawBatch;
use crate::World;
use crate::core::save::vec2::Vec2Save;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use rand::{Rng};

#[derive(PartialEq, Clone, Serialize, Deserialize)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub trait Entity: Any + Send + Sync {
    fn get_type_tag(&self) -> &'static str;
    fn get_pos(&self) -> Vec2;
    fn get_size(&self) -> Vec2;
    fn get_speed(&self) -> Vec2;

    fn tick(&mut self, _dt: f32, _world: &mut World) {
        let pos = self.get_pos();
        let speed = self.get_speed();
        self.set_pos(pos + speed);

        if rand::rng().random_range(0..100) < 1 {
            let axis = if rand::rng().random_bool(0.5) { 0 } else { 1 };
            let direction = if rand::rng().random_bool(0.5) { 1.0 } else { -1.0 };

            let new_speed = match axis {
                0 => Vec2::new(direction * 1.0, speed.y),
                1 => Vec2::new(speed.x, direction * 1.0),
                _ => speed,
            };

            self.set_speed(new_speed);
        }
    }
    fn draw(&self, batch: &mut DrawBatch);

    fn set_size(&mut self, _size: Vec2);
    fn set_pos(&mut self, pos: Vec2);
    fn set_speed(&mut self, speed: Vec2);

    fn interact(&mut self, _other: &mut dyn Entity) {}
    fn hurt(&mut self, _damage: i32, _attack_dir: Direction) {}
    fn collision(&mut self, other: &mut dyn Entity) {
        self.set_speed(Vec2::new(0.0, 0.0));
    }

    fn clone_box(&self) -> Box<dyn Entity>;
}

#[derive(Serialize, Deserialize)]
pub struct EntityData {
    pub type_tag: String,
    pub pos: Vec2Save,
    pub size: Vec2Save,
}

pub struct EntityRegistry {
    prototypes: HashMap<String, Box<dyn Entity>>,
}

impl Default for EntityRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl EntityRegistry {
    pub fn new() -> Self {
        Self {
            prototypes: HashMap::new(),
        }
    }

    pub fn register<T: Entity + 'static>(&mut self, entity: T) {
        self.prototypes.insert(entity.get_type_tag().to_string(), Box::new(entity));
    }

    pub fn create_entity_by_id(&self, type_tag: &str) -> Option<Box<dyn Entity>> {
        self.prototypes.get(type_tag).map(|proto| proto.clone_box())
    }

    pub fn deserialize_entity(&self, data: &str) -> Result<Box<dyn Entity>, String> {
        let data: EntityData = serde_json::from_str(data)
            .map_err(|e| format!("Failed to deserialize EntityData: {}", e))?;

        let prototype = self.prototypes.get(&data.type_tag)
            .ok_or_else(|| format!("Unknown entity type: {}", data.type_tag))?;

        let mut entity = prototype.clone_box();
        entity.set_pos(Vec2::from(data.pos));
        entity.set_size(Vec2::from(data.size));

        Ok(entity)
    }
}

pub trait SerializableEntity {
    fn serialize(&self) -> String;
}

impl SerializableEntity for dyn Entity {
    fn serialize(&self) -> String {
        let data = EntityData {
            type_tag: self.get_type_tag().to_string(),
            pos: Vec2Save::from(self.get_pos()),
            size: Vec2Save::from(self.get_size()),
        };
        serde_json::to_string(&data).unwrap()
    }
}
