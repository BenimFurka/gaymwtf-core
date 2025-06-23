/// Represents a biome in the game world.
///
/// A biome defines the environmental characteristics of a region, including
/// the types of ground tiles and objects that can appear there.
pub trait Biome: Send + Sync {
    /// Returns a unique identifier for this biome type.
    fn get_type_tag(&self) -> &'static str;

    /// Determines if this biome is suitable for the given environmental conditions.
    ///
    /// - `height`: The height value (0.0 to 1.0) at the location.
    /// - `moisture`: The moisture level (0.0 to 1.0) at the location.
    /// - `temperature`: The temperature (0.0 to 1.0) at the location.
    ///
    /// Returns `true` if this biome is suitable for the given conditions, `false` otherwise.
    fn is_suitable(&self, height: f64, moisture: f64, temperature: f64) -> bool;

    /// Returns the type of ground tile that should be used for this biome.
    fn get_ground_tile_type(&self) -> &'static str;
    
    /// Returns a list of object types that can spawn in this biome.
    fn get_spawnable_objects(&self) -> Vec<(&'static str, f32)>;
    
    /// Creates a boxed clone of this biome.
    fn clone_box(&self) -> Box<dyn Biome>;
}

/// A registry for managing different biome types.
pub struct BiomeRegistry {
    /// Collection of registered biome prototypes.
    prototypes: Vec<Box<dyn Biome>>,
}

impl Default for BiomeRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl BiomeRegistry {
    /// Creates a new, empty biome registry.
    pub fn new() -> Self {
        Self {
            prototypes: Vec::new(),
        }
    }

    /// Registers a new biome type with the registry.
    ///
    /// - `biome`: The biome instance to register.
    pub fn register<B: Biome + 'static>(&mut self, biome: B) {
        self.prototypes.push(Box::new(biome));
    }

    /// Finds the most suitable biome for the given environmental conditions.
    ///
    /// - `height`: The height value (0.0 to 1.0) at the location.
    /// - `moisture`: The moisture level (0.0 to 1.0) at the location.
    /// - `temperature`: The temperature (0.0 to 1.0) at the location.
    ///
    /// Returns a reference to the most suitable biome, or `None` if no suitable biome is found.
    pub fn find_biome(&self, height: f64, moisture: f64, temperature: f64) -> Option<&dyn Biome> {
        for biome in &self.prototypes {
            if biome.is_suitable(height, moisture, temperature) {
                return Some(biome.as_ref());
            }
        }
        None
    }
}
