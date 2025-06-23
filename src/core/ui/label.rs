use macroquad::prelude::*;
use super::Element;

/// A text label UI element that displays a single line of text.
///
/// The label's size is automatically determined by its text content and font size.
/// It supports basic text rendering with configurable position, color, and visibility.
pub struct Label {
    text: String,
    position: Vec2,
    font_size: u16,
    color: Color,
    visible: bool,
}

impl Label {
    /// Creates a new text label with the specified properties.
    ///
    /// - `text`: The initial text to display.
    /// - `position`: The top-left position of the label in screen coordinates.
    /// - `font_size`: The size of the font in pixels.
    /// - `color`: The color of the text.
    ///
    /// Returns a new `Label` instance with the specified properties.
    pub fn new(text: &str, position: Vec2, font_size: u16, color: Color) -> Self {
        Self {
            text: text.to_string(),
            position,
            font_size,
            color,
            visible: true,
        }
    }
    
    /// Sets the text content of the label.
    ///
    /// - `text`: The new text to display.
    pub fn set_text(&mut self, text: &str) {
        self.text = text.to_string();
    }
    
    /// Gets the current text content of the label.
    ///
    /// Returns a reference to the current text string.
    pub fn text(&self) -> &str {
        &self.text
    }
    
    /// Sets the text color of the label.
    ///
    /// - `color`: The new color for the text.
    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }
}

impl Element for Label {
    fn update(&mut self) -> bool {
        false
    }
    
    fn draw(&self) {
        if !self.visible {
            return;
        }
        
        draw_text(
            &self.text,
            self.position.x,
            self.position.y + self.font_size as f32,
            self.font_size as f32,
            self.color,
        );
    }
    
    fn bounds(&self) -> Rect {
        let text_size = measure_text(&self.text, None, self.font_size, 1.0);
        Rect::new(
            self.position.x,
            self.position.y,
            text_size.width,
            text_size.height,
        )
    }
    
    fn set_position(&mut self, position: Vec2) {
        self.position = position;
    }
    
    fn set_size(&mut self, _size: Vec2) {
    }
    
    fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }
    
    fn is_visible(&self) -> bool {
        self.visible
    }
}
