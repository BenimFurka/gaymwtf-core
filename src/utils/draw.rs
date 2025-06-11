use macroquad::{color, math::Vec2, texture::{draw_texture_ex, DrawTextureParams, Texture2D}};
use crate::log_render;

pub struct DrawBatch {
    textures: Vec<(Texture2D, Vec<(Vec2, f32, Option<Vec2>)>)>,
}

impl DrawBatch {
    pub fn new() -> Self {
        log_render!(log::Level::Trace, "Creating new DrawBatch");
        Self {
            textures: Vec::new(),
        }
    }

    pub fn add(&mut self, texture: Texture2D, pos: Vec2, size: f32, dest_size: Option<Vec2>) {
        let texture_id = texture.raw_miniquad_id();
        
        if let Some((_, instances)) = self.textures.iter_mut().find(|(t, _)| t.raw_miniquad_id() == texture_id) {
            instances.push((pos, size, dest_size));
            log_render!(log::Level::Trace, "Added to existing texture batch");
        } else {
            self.textures.push((texture, vec![(pos, size, dest_size)]));
            log_render!(log::Level::Trace, "Created new texture batch");
        }
    }

    pub fn draw(&mut self) {
        log_render!(log::Level::Debug, "Drawing batch with {} texture groups", self.textures.len());
        
        for (texture, instances) in &self.textures {
            log_render!(log::Level::Trace, "Drawing {} instances of texture", instances.len());
            
            for (pos, _size, dest_size) in instances {
                draw_texture_ex(
                    texture,
                    pos.x,
                    pos.y,
                    color::WHITE,
                    DrawTextureParams {
                        dest_size: *dest_size,
                        source: None,
                        rotation: 0.0,
                        flip_x: false,
                        flip_y: false,
                        pivot: None,
                    }
                );
            }
        }
        
        self.textures.clear();
        log_render!(log::Level::Trace, "Batch cleared");
    }

    pub fn clear(&mut self) {
        self.textures.clear();
    }
}
