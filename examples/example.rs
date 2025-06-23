use gaymwtf_core::core::world::*;
use gaymwtf_core::core::object::*;
use gaymwtf_core::core::tile::*;
use gaymwtf_core::core::biome::*;

use macroquad::prelude::*;

#[macroquad::main("My Game")]
async fn main() {
    // Initialize registries
    let tile_registry = TileRegistry::new();
    let object_registry = ObjectRegistry::new();
    let biome_registry = BiomeRegistry::new();
    
    // Create a new world
    let mut world = World::new("MyGameWorld", tile_registry, object_registry, biome_registry);
    
    let mut camera = Camera2D::from_display_rect(Rect::new(0.0, 0.0, 800.0, 600.0));
    camera.zoom.y = -camera.zoom.y;

    // Game loop
    loop {
        
        // Update game state
        world.update(camera.target, vec2(screen_width(), screen_height()));

        // Render
        clear_background(BLACK);
        set_camera(&camera);
        
        world.draw(camera.target, vec2(screen_width(), screen_height()));
        
        next_frame().await;
    }
}