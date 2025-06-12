use macroquad::prelude::*;
use gaymwtf_core::{
    Tile, TileRegistry, Entity, EntityRegistry, Biome, BiomeRegistry, Chunk, World, DrawBatch, TILE_SIZE, CHUNK_SIZE, CHUNK_PIXELS
};
use ::rand::rng;
use ::rand::Rng;

// --- Concrete Tile Implementations ---

#[derive(Clone)]
struct Air {
    pos: Vec2,
}

impl Tile for Air {
    fn get_type_tag(&self) -> &'static str { "air" }
    fn get_pos(&self) -> Vec2 { self.pos }
    fn get_size(&self) -> Vec2 { vec2(TILE_SIZE, TILE_SIZE) }

    fn set_pos(&mut self, pos: Vec2) { self.pos = pos; }
    fn set_size(&mut self, _size: Vec2) {}

    fn tick(&mut self, _dt: f32, _world: &mut World) {}
    fn draw(&self, _batch: &mut DrawBatch, _pos: Vec2) { }

    fn clone_box(&self) -> Box<dyn Tile> { Box::new(self.clone()) }
}

#[derive(Clone)]
struct Stone {
    pos: Vec2,
    texture: Texture2D,
}

impl Tile for Stone {
    fn get_type_tag(&self) -> &'static str { "stone" }
    fn get_pos(&self) -> Vec2 { self.pos }
    fn get_size(&self) -> Vec2 { vec2(TILE_SIZE, TILE_SIZE) }

    fn set_pos(&mut self, pos: Vec2) { self.pos = pos; }
    fn set_size(&mut self, _size: Vec2) {}

    fn tick(&mut self, _dt: f32, _world: &mut World) {}
    fn draw(&self, batch: &mut DrawBatch, pos: Vec2) {
        batch.add(self.texture.clone(), pos, TILE_SIZE, None);
    }

    fn clone_box(&self) -> Box<dyn Tile> { Box::new(self.clone()) }
}

// --- Concrete Entity Implementations ---

#[derive(Clone)]
struct Mob {
    pos: Vec2,
    speed: Vec2,
    size: Vec2,
    texture: Texture2D,
}

impl Entity for Mob {
    fn get_type_tag(&self) -> &'static str { "mob" }
    fn get_pos(&self) -> Vec2 { self.pos }
    fn get_size(&self) -> Vec2 { vec2(TILE_SIZE, TILE_SIZE) }
    fn get_speed(&self) -> Vec2 { self.speed}

    fn set_pos(&mut self, pos: Vec2) { self.pos = pos; }
    fn set_size(&mut self, _size: Vec2) {}
    fn set_speed(&mut self, speed: Vec2) { self.speed = speed; }

    fn draw(&self, batch: &mut DrawBatch) {
        batch.add(self.texture.clone(), self.pos, TILE_SIZE, Some(self.size));
    }

    fn clone_box(&self) -> Box<dyn Entity> { Box::new(self.clone()) }
}

// --- Concrete Biome Implementations ---

#[derive(Clone)]
struct Plains;

impl Biome for Plains {
    fn get_type_tag(&self) -> &'static str { "plains" }
    fn is_suitable(&self, _height: f64, _moisture: f64, _temperature: f64) -> bool { true }
    fn get_ground_tile_type(&self) -> &'static str { "stone" }
    fn get_spawnable_entities(&self) -> Vec<(&'static str, f32)> { vec![("mob", 0.05)] }
    fn clone_box(&self) -> Box<dyn Biome> { Box::new(self.clone()) }
}

fn generate_chunk(pos: Vec2, tile_registry: &TileRegistry, biome_registry: &BiomeRegistry, entity_registry: &EntityRegistry) -> Chunk {
    let mut chunk = Chunk::new(pos);
    let biome = biome_registry.find_biome(0.0, 0.0, 0.0).unwrap(); 
    let mut rang = rng();

    let chunk_world_pos = pos * CHUNK_PIXELS as f32;

    for y in 0..CHUNK_SIZE {
        for x in 0..CHUNK_SIZE {
            let tile_type = biome.get_ground_tile_type();
            let mut tile = tile_registry.create_tile_by_id(tile_type).unwrap();

            let tile_pos = chunk_world_pos + vec2(x as f32 * TILE_SIZE, y as f32 * TILE_SIZE);
            tile.set_pos(tile_pos);
            chunk.tiles.push(tile); 

            for (entity_type, chance) in biome.get_spawnable_entities() {
                if rang.random::<f32>() < chance {
                    if let Some(mut entity) = entity_registry.create_entity_by_id(entity_type) {
                        entity.set_pos(tile_pos);
                        chunk.entities.push(entity);
                    }
                }
            }
        }
    }
    chunk
}

async fn setup() -> World {
    let mut tile_registry = TileRegistry::new();
    tile_registry.register(Air { pos: Vec2::ZERO });

    let stone_texture = Texture2D::from_rgba8(16, 16, &[128; 16 * 16 * 4]);
    tile_registry.register(Stone { pos: Vec2::ZERO, texture: stone_texture });

    let mut entity_registry = EntityRegistry::new();

    let mob_texture = Texture2D::from_rgba8(16, 16, &[255; 16 * 16 * 4]);
    entity_registry.register(Mob { pos: Vec2::ZERO, speed: Vec2::ZERO, size: Vec2::new(TILE_SIZE, TILE_SIZE), texture: mob_texture });

    let mut biome_registry = BiomeRegistry::new();
    biome_registry.register(Plains);

    let mut world = World::new("test-world", tile_registry, entity_registry, biome_registry);

    for y in -2..=2 {
        for x in -2..=2 {
            let chunk_pos = vec2(x as f32, y as f32);
            let chunk = generate_chunk(chunk_pos, &world.tile_registry, &world.biome_registry, &world.entity_registry);
            world.add_chunk(chunk);
        }
    }

    world
}

#[macroquad::main("gaymwtf-test")]
async fn main() {
    let mut world = setup().await;
    let mut camera = Camera2D::from_display_rect(Rect::new(0.0, 0.0, 800.0, 600.0));
    camera.zoom.y = -camera.zoom.y;

    loop {
        // --- Input ---
        if is_key_down(KeyCode::Right) { camera.target.x += 10.0; }
        if is_key_down(KeyCode::Left) { camera.target.x -= 10.0; }
        if is_key_down(KeyCode::Up) { camera.target.y -= 10.0; }
        if is_key_down(KeyCode::Down) { camera.target.y += 10.0; }

        // --- Update ---
        world.update(camera.target, vec2(screen_width(), screen_height()));

        // --- Draw ---
        clear_background(SKYBLUE);
        set_camera(&camera);

        world.draw(camera.target, vec2(screen_width(), screen_height()));

        set_default_camera();
        draw_text(&format!("FPS: {}", get_fps()), 10.0, 20.0, 20.0, WHITE);
        draw_text(&format!("Chunks: {}", world.chunks.len()), 10.0, 40.0, 20.0, WHITE);

        next_frame().await
    }
}
