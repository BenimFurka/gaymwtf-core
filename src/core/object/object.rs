use std::any::Any;
use macroquad::math::Vec2;
use crate::utils::draw::DrawBatch;
use crate::World;
use crate::core::save::vec2::Vec2Save;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use macroquad::prelude::vec2;

#[derive(PartialEq, Clone, Serialize, Deserialize)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub trait Object: Any + Send + Sync {
    fn get_type_tag(&self) -> &'static str;
    fn get_pos(&self) -> Vec2;
    fn get_size(&self) -> Vec2;
    fn get_velocity(&self) -> Vec2;

    fn tick(&mut self, _dt: f32, _world: &mut World) {}
    fn draw(&self, batch: &mut DrawBatch);

    fn set_size(&mut self, _size: Vec2);
    fn set_pos(&mut self, pos: Vec2);
    fn set_velocity(&mut self, velocity: Vec2);

    fn interact(&mut self, _other: &mut dyn Object) {}
    fn hurt(&mut self, _damage: i32, _attack_dir: Direction) {}
    fn collision(&mut self, other: &mut dyn Object) {
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
    fn clone_box(&self) -> Box<dyn Object>;
}

#[derive(Serialize, Deserialize)]
pub struct ObjectData {
    pub type_tag: String,
    pub pos: Vec2Save,
    pub size: Vec2Save,
}

pub struct ObjectRegistry {
    prototypes: HashMap<String, Box<dyn Object>>,
}

impl Default for ObjectRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl ObjectRegistry {
    pub fn new() -> Self {
        Self {
            prototypes: HashMap::new(),
        }
    }

    pub fn register<T: Object + 'static>(&mut self, obj: T) {
        self.prototypes.insert(obj.get_type_tag().to_string(), Box::new(obj));
    }

    pub fn create_object_by_id(&self, type_tag: &str) -> Option<Box<dyn Object>> {
        self.prototypes.get(type_tag).map(|proto| proto.clone_box())
    }

    pub fn deserialize_object(&self, data: &str) -> Result<Box<dyn Object>, String> {
        let data: ObjectData = serde_json::from_str(data)
            .map_err(|e| format!("Failed to deserialize ObjectData: {}", e))?;

        let prototype = self.prototypes.get(&data.type_tag)
            .ok_or_else(|| format!("Unknown object type: {}", data.type_tag))?;

        let mut obj = prototype.clone_box();
        obj.set_pos(Vec2::from(data.pos));
        obj.set_size(Vec2::from(data.size));

        Ok(obj)
    }
}

pub trait SerializableObject {
    fn serialize(&self) -> String;
}

impl SerializableObject for dyn Object {
    fn serialize(&self) -> String {
        let data = ObjectData {
            type_tag: self.get_type_tag().to_string(),
            pos: Vec2Save::from(self.get_pos()),
            size: Vec2Save::from(self.get_size()),
        };
        serde_json::to_string(&data).unwrap()
    }
}
