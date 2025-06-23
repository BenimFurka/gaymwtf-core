use gaymwtf_core::core::ui::*;
use gaymwtf_core::DrawBatch;
use macroquad::prelude::*;
use std::collections::HashMap;

// Toggle Button
struct ToggleButton {
    button: Button,
    is_on: bool,
    on_text: String,
    off_text: String,
}

impl ToggleButton {
    fn new(bounds: Rect, on_text: &str, off_text: &str) -> Self {
        Self {
            button: Button::new(off_text, bounds),
            is_on: false,
            on_text: on_text.to_string(),
            off_text: off_text.to_string(),
        }
    }

    fn update(&mut self) -> bool {
        if self.button.update() && self.button.was_clicked() {
            self.is_on = !self.is_on;
            self.button.set_text(if self.is_on { &self.on_text } else { &self.off_text });
            true
        } else {
            false
        }
    }

    fn draw(&self) {
        self.button.draw();
    }

    fn is_on(&self) -> bool {
        self.is_on
    }
}

// Main Menu
struct MainMenu {
    title: Label,
    start_button: Button,
    options_button: Button,
    quit_button: Button,
}

impl MainMenu {
    fn new() -> Self {
        let screen_center = Vec2::new(screen_width() / 2.0, screen_height() / 2.0);
        
        Self {
            title: Label::new("MAIN MENU", vec2(screen_center.x - 100.0, 100.0), 40, WHITE),
            start_button: Button::new("Start Game", Rect::new(screen_center.x - 100.0, screen_center.y - 50.0, 200.0, 40.0)),
            options_button: Button::new("Options", Rect::new(screen_center.x - 100.0, screen_center.y + 10.0, 200.0, 40.0)),
            quit_button: Button::new("Quit", Rect::new(screen_center.x - 100.0, screen_center.y + 70.0, 200.0, 40.0)),
        }
    }
}

impl Menu for MainMenu {
    fn update(&mut self, _dt: f32) -> MenuAction {
        self.start_button.update();
        self.options_button.update();
        self.quit_button.update();
        
        if self.start_button.was_clicked() {
            self.start_button.reset_click();
            return MenuAction::ChangeState("game".to_string());
        }
        
        if self.options_button.was_clicked() {
            self.options_button.reset_click();
            return MenuAction::ChangeState("options".to_string());
        }
        
        if self.quit_button.was_clicked() {
            self.quit_button.reset_click();
            return MenuAction::Quit;
        }
        
        MenuAction::None
    }

    fn draw(&mut self, _batch: &mut DrawBatch) {
        clear_background(Color::new(0.1, 0.1, 0.2, 1.0));
        
        self.title.draw();
        self.start_button.draw();
        self.options_button.draw();
        self.quit_button.draw();
    }

    fn name(&self) -> &str {
        "main"
    }
}

// Options Menu
struct OptionsMenu {
    title: Label,
    sound_toggle: ToggleButton,
    back_button: Button,
}

impl OptionsMenu {
    fn new() -> Self {
        let screen_center = vec2(screen_width() / 2.0, screen_height() / 2.0);
        
        Self {
            title: Label::new("Options", vec2(screen_center.x - 80.0, 100.0), 40, WHITE),
            sound_toggle: ToggleButton::new(
                Rect::new(screen_center.x - 100.0, screen_center.y - 50.0, 200.0, 40.0),
                "Sound: ON",
                "Sound: OFF"
            ),
            back_button: Button::new("Back", Rect::new(screen_center.x - 100.0, screen_center.y + 50.0, 200.0, 40.0)),
        }
    }
}

impl Menu for OptionsMenu {
    fn update(&mut self, _dt: f32) -> MenuAction {
        self.back_button.update();
        self.sound_toggle.update();
        
        if self.back_button.was_clicked() {
            self.back_button.reset_click();
            return MenuAction::ChangeState("main".to_string());
        }
        
        if self.sound_toggle.button.was_clicked() {
            self.sound_toggle.button.reset_click();
        }
        
        MenuAction::None
    }

    fn draw(&mut self, _batch: &mut DrawBatch) {
        clear_background(Color::new(0.1, 0.2, 0.1, 1.0));
        
        self.title.draw();
        self.sound_toggle.draw();
        self.back_button.draw();
        
        let hint = if self.sound_toggle.is_on() {
            "Sound is currently enabled!"
        } else {
            "Sound is currently disabled"
        };
        
        draw_text(
            hint,
            screen_width() / 2.0 - 100.0,
            screen_height() - 50.0,
            20.0,
            LIGHTGRAY,
        );
    }

    fn name(&self) -> &str {
        "options"
    }
}

#[macroquad::main("UI Example")]
async fn main() {
    // Create menus
    let mut menus: HashMap<String, Box<dyn Menu>> = HashMap::new();
    menus.insert("main".to_string(), Box::new(MainMenu::new()));
    menus.insert("options".to_string(), Box::new(OptionsMenu::new()));
    
    let mut current_menu = "main".to_string();
    
    let mut draw_batch = DrawBatch::new();
    
    // Main game loop
    loop {
        clear_background(BLACK);
        
        let menu_action = {
            if let Some(menu) = menus.get_mut(&current_menu) {
                menu.update(get_frame_time())
            } else {
                MenuAction::Quit
            }
        };
        
        // Handle menu action
        match menu_action {
            MenuAction::None => {}
            MenuAction::ChangeState(state) => {
                current_menu = state;
            }
            MenuAction::Quit => {
                break;
            }
        }
        
        if let Some(menu) = menus.get_mut(&current_menu) {
            menu.draw(&mut draw_batch);
        } else {
            break;
        }
        
        next_frame().await;
    }
}