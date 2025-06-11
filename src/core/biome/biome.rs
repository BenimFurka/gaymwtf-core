

pub trait Biome: Send + Sync {
    fn get_type_tag(&self) -> &'static str;
    fn is_suitable(&self, height: f64, moisture: f64, temperature: f64) -> bool;
    fn get_ground_tile_type(&self) -> &'static str;
    fn get_spawnable_entities(&self) -> Vec<(&'static str, f32)>;
    fn clone_box(&self) -> Box<dyn Biome>;
}

pub struct BiomeRegistry {
    prototypes: Vec<Box<dyn Biome>>,
}

impl Default for BiomeRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl BiomeRegistry {
    pub fn new() -> Self {
        Self {
            prototypes: Vec::new(),
        }
    }

    pub fn register<B: Biome + 'static>(&mut self, biome: B) {
        self.prototypes.push(Box::new(biome));
    }

    pub fn find_biome(&self, height: f64, moisture: f64, temperature: f64) -> Option<&dyn Biome> {
        for biome in &self.prototypes {
            if biome.is_suitable(height, moisture, temperature) {
                return Some(biome.as_ref());
            }
        }
        None
    }
}
