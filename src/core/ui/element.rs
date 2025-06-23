use macroquad::prelude::*;

/// Base trait for all UI elements.
///
/// This trait defines the common interface that all UI elements must implement,
/// including methods for updating, drawing, and handling interactions.
pub trait Element {
    /// Updates the element's state.
    ///
    /// This method is called once per frame to update the element's internal state.
    ///
    /// Returns `true` if the element's state changed and requires a redraw, `false` otherwise.
    fn update(&mut self) -> bool;
    
    /// Draws the element on the screen.
    ///
    /// This method is responsible for rendering the element's visual representation.
    /// It should only be called if `is_visible()` returns `true`.
    fn draw(&self);
    
    /// Checks if the element contains the specified point.
    ///
    /// This is used for hit testing, such as determining if the mouse is over the element.
    ///
    /// - `point`: The point to check, in screen coordinates.
    ///
    /// Returns `true` if the point is within the element's bounds, `false` otherwise.
    fn contains(&self, point: Vec2) -> bool {
        self.bounds().contains(point)
    }
    
    /// Gets the element's bounding rectangle in screen coordinates.
    ///
    /// Returns a `Rect` representing the element's position and size on screen.
    fn bounds(&self) -> Rect;
    
    /// Sets the element's position.
    ///
    /// - `position`: The new position in screen coordinates.
    fn set_position(&mut self, position: Vec2);
    
    /// Sets the element's size.
    ///
    /// - `size`: The new size in pixels.
    fn set_size(&mut self, size: Vec2);
    
    /// Sets the element's visibility.
    ///
    /// - `visible`: `true` to make the element visible, `false` to hide it.
    fn set_visible(&mut self, visible: bool);
    
    /// Gets the element's visibility state.
    ///
    /// Returns `true` if the element is visible, `false` otherwise.
    fn is_visible(&self) -> bool;
}
