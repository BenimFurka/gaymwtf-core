/// Size of a single tile in world units (pixels).
pub const TILE_SIZE: f32 = 16.0;

/// Number of tiles along one edge of a chunk.
pub const CHUNK_SIZE: usize = 16;

/// Size of a chunk in world units (pixels).
pub const CHUNK_PIXELS: f32 = TILE_SIZE * CHUNK_SIZE as f32;

/// Margin around the viewport in which objects become active.
pub const OBJECT_ACTIVATION_MARGIN: f32 = 100.0;
