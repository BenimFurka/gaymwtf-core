use macroquad::math::Vec2;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Vec2Save {
    pub x: f32,
    pub y: f32,
}

impl From<Vec2> for Vec2Save {
    fn from(vec: Vec2) -> Self {
        Vec2Save {
            x: vec.x,
            y: vec.y
        }
    }
}

impl From<Vec2Save> for Vec2 {
    fn from(save: Vec2Save) -> Self {
        Vec2::new(save.x, save.y)
    }
}
