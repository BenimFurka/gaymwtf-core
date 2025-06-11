use crate::utils::draw::DrawBatch;

#[derive(Debug, PartialEq, Eq)]
pub enum MenuAction {
    /// No action was taken
    None,
    /// Request to change the game state
    ChangeState(String),
    /// Request to quit the application
    Quit,
}

pub trait Menu {
    fn update(&mut self, dt: f32) -> MenuAction;

    fn draw(&self, batch: &mut DrawBatch);

    fn name(&self) -> &str;
}
