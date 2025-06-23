use macroquad::math::Vec2;
use serde::{Deserialize, Serialize};

/// A serializable version of `macroquad::math::Vec2`.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Vec2Save {
    /// The x-coordinate of the vector.
    pub x: f32,
    /// The y-coordinate of the vector.
    pub y: f32,
}

impl From<Vec2> for Vec2Save {
    /// Converts a `Vec2` to a `Vec2Save`.
    fn from(vec: Vec2) -> Self {
        Vec2Save {
            x: vec.x,
            y: vec.y
        }
    }
}

impl From<Vec2Save> for Vec2 {
    /// Converts a `Vec2Save` back to a `Vec2`.
    fn from(save: Vec2Save) -> Self {
        Vec2::new(save.x, save.y)
    }
}
