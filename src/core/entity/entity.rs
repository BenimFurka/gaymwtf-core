use std::any::Any;
use macroquad::math::Vec2;
use crate::utils::draw::DrawBatch;
use crate::World;
use crate::core::save::vec2::Vec2Save;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use rand::{Rng};
use macroquad::prelude::vec2;

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
    fn get_velocity(&self) -> Vec2;

    fn tick(&mut self, _dt: f32, _world: &mut World) {
        let pos = self.get_pos();
        let velocity = self.get_velocity();
        self.set_pos(pos + velocity);

        if rand::rng().random_range(0..100) < 10 {
            let axis = if rand::rng().random_bool(0.5) { 0 } else { 1 };
            let direction = if rand::rng().random_bool(0.5) { 1.0 } else { -1.0 };

            let new_velocity = match axis {
                0 => Vec2::new(direction * 1.0, velocity.y),
                1 => Vec2::new(velocity.x, direction * 1.0),
                _ => velocity,
            };

            self.set_velocity(new_velocity);
        }
    }
    fn draw(&self, batch: &mut DrawBatch);

    fn set_size(&mut self, _size: Vec2);
    fn set_pos(&mut self, pos: Vec2);
    fn set_velocity(&mut self, velocity: Vec2);

    fn interact(&mut self, _other: &mut dyn Entity) {}
    fn hurt(&mut self, _damage: i32, _attack_dir: Direction) {}
    fn collision(&mut self, other: &mut dyn Entity) {
        let buffer = 1.0;
        let self_pos = self.get_pos();
        let self_size = self.get_size();
        let other_pos = other.get_pos();
        let other_size = other.get_size();
        
        let self_bounds = (
            self_pos + vec2(buffer, buffer),
            self_pos + self_size - vec2(buffer, buffer)
        );
        
        let other_bounds = (
            other_pos + vec2(buffer, buffer),
            other_pos + other_size - vec2(buffer, buffer)
        );
        
        if self_bounds.0.x < other_bounds.1.x &&
           self_bounds.1.x > other_bounds.0.x &&
           self_bounds.0.y < other_bounds.1.y &&
           self_bounds.1.y > other_bounds.0.y {
            let mut velocity = self.get_velocity();
            
            let x_overlap = (self_bounds.1.x - other_bounds.0.x).min(other_bounds.1.x - self_bounds.0.x);
            let y_overlap = (self_bounds.1.y - other_bounds.0.y).min(other_bounds.1.y - self_bounds.0.y);
            
            if x_overlap < y_overlap {
                velocity.x = 0.0;
            } else if x_overlap > y_overlap {
                velocity.y = 0.0;
            } else {
                velocity.x = 0.0;
                velocity.y = 0.0;
            }
            
            self.set_velocity(velocity);
        }
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
