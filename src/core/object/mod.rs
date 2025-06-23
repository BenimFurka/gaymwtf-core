use std::any::Any;
use macroquad::math::Vec2;
use crate::utils::draw::DrawBatch;
use crate::World;
use crate::core::save::Vec2Save;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use macroquad::prelude::vec2;

/// Represents the four cardinal directions used for movement and facing.
#[derive(PartialEq, Clone, Serialize, Deserialize)]
pub enum Direction {
    /// Facing or moving upward
    Up,
    /// Facing or moving downward
    Down,
    /// Facing or moving left
    Left,
    /// Facing or moving right
    Right,
}

/// Represents a dynamic game object that can move and interact with the world.
/// Objects are entities that can have behaviors, move around, and interact with
/// both tiles and other objects. Examples include players, enemies, and other objects.
pub trait Object: Any + Send + Sync {
    /// Returns a unique identifier for the object type
    fn get_type_tag(&self) -> &'static str;
    
    /// Returns the position of the object in world coordinates
    fn get_pos(&self) -> Vec2;
    
    /// Returns the size of the object in world units
    fn get_size(&self) -> Vec2;
    
    /// Returns the current velocity of the object
    fn get_velocity(&self) -> Vec2;

    /// Called every frame to update the object's state
    /// 
    /// - `dt`: Time elapsed since the last frame in seconds
    /// - `world`: Reference to the game world for interaction
    fn tick(&mut self, _dt: f32, _world: &mut World) { }
    
    /// Draws the object on the screen
    /// 
    /// - `batch`: The draw batch to add drawing commands to
    fn draw(&self, batch: &mut DrawBatch);

    /// Sets the size of the object in world units
    fn set_size(&mut self, _size: Vec2);
    
    /// Sets the position of the object in world coordinates
    fn set_pos(&mut self, pos: Vec2);
    
    /// Sets the velocity of the object
    fn set_velocity(&mut self, velocity: Vec2);

    /// Called when another object right-clicks on this object.  
    /// 
    /// - `other`: The object that initiated the right-click.
    fn on_right_interact(&mut self, _other: &mut dyn Object) { }  

    /// Called when another object left-clicks on this object.  
    /// 
    /// - `other`: The object that initiated the left-click.
    fn on_left_interact(&mut self, _other: &mut dyn Object) { }  

    /// Called when this object collides with another object
    /// Handles the physics of the collision
    /// 
    /// - `other`: The other object involved in the collision
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
    
    /// Creates a boxed clone of this object
    fn clone_box(&self) -> Box<dyn Object>;
}

/// Serializable data structure representing an object's state.
/// Used for saving and loading object states from disk.
#[derive(Serialize, Deserialize)]
pub struct ObjectData {
    /// Unique identifier of the object's type
    pub type_tag: String,
    /// Position of the object in world coordinates
    pub pos: Vec2Save,
    /// Size of the object in world units
    pub size: Vec2Save,
}

/// Manages the registration and instantiation of object types.
/// Maintains a collection of object prototypes that can be cloned to create new instances.
pub struct ObjectRegistry {
    /// Map of object type tags to their prototype instances
    prototypes: HashMap<String, Box<dyn Object>>,
}

impl Default for ObjectRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl ObjectRegistry {
    /// Creates a new, empty ObjectRegistry
    pub fn new() -> Self {
        Self {
            prototypes: HashMap::new(),
        }
    }

    /// Registers a new object type with the registry
    /// 
    /// - `obj`: The prototype object to register
    /// - `T`: Type parameter that implements both Object and 'static
    pub fn register<T: Object + 'static>(&mut self, obj: T) {
        self.prototypes.insert(obj.get_type_tag().to_string(), Box::new(obj));
    }

    /// Creates a new instance of an object by its type tag
    /// 
    /// - `type_tag`: The type identifier of the object to create
    /// 
    /// Returns `Some(boxed_object)` if found, `None` otherwise
    pub fn create_object_by_id(&self, type_tag: &str) -> Option<Box<dyn Object>> {
        self.prototypes.get(type_tag).map(|proto| proto.clone_box())
    }

    /// Deserializes an object from a JSON string
    /// 
    /// - `data`: JSON string containing serialized object data
    /// 
    /// Returns a boxed object on success, or an error message on failure
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

/// Trait for objects that can be serialized to and from strings.
/// Primarily used for saving and loading game states.
pub trait SerializableObject {
    /// Serializes the object to a JSON string
    fn serialize(&self) -> String;
}

// Default implementation of SerializableObject for any type implementing Object
impl SerializableObject for dyn Object {
    /// Serializes the object's data to a JSON string
    /// Includes type tag, position, and size information
    fn serialize(&self) -> String {
        let data = ObjectData {
            type_tag: self.get_type_tag().to_string(),
            pos: Vec2Save::from(self.get_pos()),
            size: Vec2Save::from(self.get_size()),
        };
        serde_json::to_string(&data).unwrap()
    }
}
