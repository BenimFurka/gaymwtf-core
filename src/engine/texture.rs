use macroquad::prelude::*;
use std::fs;
use image;
use anyhow::{Context, Result};

/// Loads a file from the given path synchronously.
///
/// - `path`: The file path to load.
///
/// Returns `Result<Vec<u8>>` containing the file bytes on success, or an error on failure.
pub fn load_file_sync(path: &str) -> Result<Vec<u8>> {
    fs::read(path).with_context(|| format!("Failed to read file: {}", path))
}

/// Loads a texture from an image file synchronously.
///
/// - `path`: The file path of the image to load.
///
/// Returns `Result<Texture2D>` containing the loaded texture on success, or an error on failure.
pub fn load_texture_sync(path: &str) -> Result<Texture2D> {
    let bytes = load_file_sync(path)?;
    let image = image::load_from_memory(&bytes)
        .with_context(|| format!("Failed to decode image from file: {}", path))?;
    let rgba_image = image.to_rgba8();
    let (width, height) = rgba_image.dimensions();
    let texture = Texture2D::from_rgba8(width as u16, height as u16, &rgba_image);
    texture.set_filter(FilterMode::Nearest);
    Ok(texture)
}
