use macroquad::prelude::*;
use super::Element;

/// Represents the visual and interactive state of a button.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonState {
    /// The button is in its default state.
    Normal,
    /// The mouse is hovering over the button.
    Hovered,
    /// The button is currently being pressed.
    Pressed,
    /// The button is disabled and cannot be interacted with.
    Disabled,

}

/// A clickable button UI element.
///
/// This component handles user interaction and visual feedback for button presses.
/// It supports different visual states (normal, hovered, pressed, disabled)
/// and can be used to trigger actions when clicked.
pub struct Button {
    /// The text displayed on the button.
    text: String,
    /// The position and size of the button in screen coordinates.
    bounds: Rect,
    /// The current visual state of the button.
    state: ButtonState,
    /// Whether the button is currently visible.
    visible: bool,
    /// Whether the button was pressed since the last check.
    was_pressed: bool,
}

impl Button {
    /// Creates a new button with the specified text and bounds.
    ///
    /// - `text`: The text to display on the button.
    /// - `bounds`: The position and size of the button in screen coordinates.
    ///
    /// Returns a new `Button` instance in the `Normal` state.
    pub fn new(text: &str, bounds: Rect) -> Self {
        Self {
            text: text.to_string(),
            bounds,
            state: ButtonState::Normal,
            visible: true,
            was_pressed: false,
        }
    }
    
    /// Sets the text displayed on the button.
    ///
    /// - `text`: The new text to display.
    pub fn set_text(&mut self, text: &str) {
        self.text = text.to_string();
    }
    
    /// Gets the current text displayed on the button.
    ///
    /// Returns a reference to the button's text content.
    pub fn text(&self) -> &str {
        &self.text
    }
    
    /// Checks if the button was clicked since the last check.
    ///
    /// This method returns `true` if the button was clicked (pressed and released)
    /// since the last time this method was called or since `reset_click()` was called.
    ///
    /// Returns `true` if the button was clicked, `false` otherwise.
    pub fn was_clicked(&self) -> bool {
        if !self.visible || self.state == ButtonState::Disabled {
            return false;
        }
        self.was_pressed
    }
    
    /// Resets the clicked state of the button.
    ///
    /// After calling this method, `was_clicked()` will return `false` until
    /// the button is clicked again.
    pub fn reset_click(&mut self) {
        if self.was_pressed {
            self.was_pressed = false;
        }
    }
}

impl Element for Button {
    fn update(&mut self) -> bool {
        if !self.visible || self.state == ButtonState::Disabled {
            return false;
        }
        
        let mouse_pos = mouse_position().into();
        let is_hovered = self.bounds.contains(mouse_pos);
        let is_pressed = is_mouse_button_pressed(MouseButton::Left);
        
        let mut state_changed = false;
        
        match self.state {
            ButtonState::Normal if is_hovered && is_pressed => {
                self.state = ButtonState::Pressed;
                state_changed = true;
            }
            ButtonState::Pressed if !is_pressed => {
                if is_hovered {
                    self.was_pressed = true;
                    self.state = ButtonState::Hovered;
                } else {
                    self.state = ButtonState::Normal;
                }
                state_changed = true;
            }
            ButtonState::Hovered if is_hovered && is_pressed => {
                self.state = ButtonState::Pressed;
                state_changed = true;
            }
            ButtonState::Hovered if !is_hovered => {
                self.state = ButtonState::Normal;
                state_changed = true;
            }
            ButtonState::Normal if is_hovered => {
                self.state = ButtonState::Hovered;
                state_changed = true;
            }
            _ => {}
        }
        
        state_changed
    }
    
    fn draw(&self) {
        if !self.visible {
            return;
        }
        
        let bg_color = match self.state {
            ButtonState::Normal => GRAY,
            ButtonState::Hovered => LIGHTGRAY,
            ButtonState::Pressed => DARKGRAY,
            ButtonState::Disabled => Color::new(0.3, 0.3, 0.3, 0.5),
        };
        
        draw_rectangle(
            self.bounds.x,
            self.bounds.y,
            self.bounds.w,
            self.bounds.h,
            bg_color,
        );
        
        let text_size = measure_text(&self.text, None, 20, 1.0);
        let text_x = self.bounds.x + (self.bounds.w - text_size.width) / 2.0;
        let text_y = self.bounds.y + (self.bounds.h + text_size.height) / 2.0;
        
        let text_color = if self.state == ButtonState::Disabled {
            GRAY
        } else {
            WHITE
        };
        
        draw_text(
            &self.text,
            text_x,
            text_y,
            20.0,
            text_color,
        );
        
        draw_rectangle_lines(
            self.bounds.x,
            self.bounds.y,
            self.bounds.w,
            self.bounds.h,
            2.0,
            if self.state == ButtonState::Pressed { DARKGRAY } else { BLACK },
        );
    }
    
    fn bounds(&self) -> Rect {
        self.bounds
    }
    
    fn set_position(&mut self, position: Vec2) {
        self.bounds.x = position.x;
        self.bounds.y = position.y;
    }
    
    fn set_size(&mut self, size: Vec2) {
        self.bounds.w = size.x;
        self.bounds.h = size.y;
    }
    
    fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }
    
    fn is_visible(&self) -> bool {
        self.visible
    }
}
