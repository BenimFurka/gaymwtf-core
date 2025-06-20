# GaymWTF Core

[![Rust](https://img.shields.io/badge/rust-stable-orange.svg)](https://www.rust-lang.org/)
[![Macroquad](https://img.shields.io/badge/macroquad-0.4.x-blue)](https://github.com/not-fl3/macroquad)

[![Crates.io](https://img.shields.io/crates/v/gaymwtf-core)](https://crates.io/crates/gaymwtf-core)
[![License](https://img.shields.io/badge/License-GPLv3-blue.svg)](LICENSE)
![Status](https://img.shields.io/badge/status-active-brightgreen)

A modular 2D game engine and framework built with Rust and Macroquad.

## Features

- **Entity-Component System**: Flexible entity management with support for custom components
- **Chunk-based World**: Efficient world management with chunk loading and unloading
- **Tile System**: Support for tile-based maps and environments
- **Biome System**: Framework for creating and managing different game biomes
- **Menu System**: Simple menu system for creating in-game menus
- **Serialization**: Built-in support for saving and loading game state
- **Rendering**: 2D rendering powered by Macroquad

## Getting Started

### Prerequisites

- Rust (latest stable version recommended)
- Cargo (Rust's package manager)

### Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
gaymwtf-core = "0.1.0"
```

Or run:

```bash
cargo add gaymwtf-core
```

## Usage

Basic example of setting up a game world:

```rust
use gaymwtf_core::*;
use macroquad::prelude::*;

#[macroquad::main("My Game")]
async fn main() {
    // Initialize registries
    let tile_registry = TileRegistry::new();
    let entity_registry = EntityRegistry::new();
    let biome_registry = BiomeRegistry::new();
    
    // Create a new world
    let mut world = World::new("MyGameWorld", tile_registry, entity_registry, biome_registry);
    
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
```

## Examples

### Test Project

The repository includes a `gaymwtf-test` example that serves as a working example of how to use the game engine. This test project demonstrates:

- Setting up custom tiles, entities, and biomes
- World generation
- Basic game loop implementation
- Rendering and input handling

To run the test project:

```bash
cargo run --example gaymwtf-test
```

## Project Structure

- `src/core/`: Core game systems (world, entities, tiles, biomes)
  - `world/`: World management
  - `chunk/`: Chunk system
  - `entity/`: Entity system and implementations
  - `tile/`: Tile system and implementations
  - `biome/`: Biome system and implementations
  - `menu/`: Menu system
  - `save/`: Vec2Save
- `src/engine/`: Rendering and other engine-specific code
- `src/utils/`: Utility functions and helpers

## License

This project is licensed under the GPL-3.0 License - see the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request or open an Issue.

## Acknowledgments

- Built with [Macroquad](https://github.com/not-fl3/macroquad)
- Uses [Serde](https://serde.rs/) for serialization
