use crate::utils::draw::DrawBatch;

/// Represents an action that can be returned by a menu.
///
/// This enum is used to communicate user interactions with the menu
/// back to the game state manager.
#[derive(Debug, PartialEq, Eq)]
pub enum MenuAction {
    /// No action was taken during this update.
    None,
    /// Request to change the game state to the specified state.
    ///
    /// The string parameter should match the name of the target game state.
    ChangeState(String),
    /// Request to quit the application.
    Quit,
}

/// A trait representing a menu in the game's user interface.
///
/// This trait defines the interface that all menu implementations must provide.
/// Menus are responsible for handling user input and rendering their UI elements.
pub trait Menu {
    /// Updates the menu's state and processes user input.
    ///
    /// - `dt`: The time delta since the last update, in seconds.
    ///
    /// Returns a `MenuAction` indicating what action (if any) should be taken as a result of this update.
    fn update(&mut self, dt: f32) -> MenuAction;

    /// Draws the menu using the provided draw batch.
    ///
    /// - `batch`: The draw batch to use for rendering.
    fn draw(&mut self, batch: &mut DrawBatch);

    /// Gets the name of this menu.
    ///
    /// This is typically used to identify the menu when switching between different menus.
    ///
    /// Returns a string slice containing the menu's name.
    fn name(&self) -> &str;
}
