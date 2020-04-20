use crate::serialize::MaterialConfig;
use super::*;

#[derive(Debug, Default, Clone)]
pub struct Material {
    pub color: Coloration,
    pub albedo: f32,
    pub reflectivity: f32
}

impl Material {
    pub fn new(color: Coloration) -> Material {
        Material {
            color,
            albedo: Default::default(),
            reflectivity: Default::default()
        }
    }
}

impl Material {
    pub fn albedo(mut self, albedo: f32) -> Self {
        self.albedo = albedo;
        self
    }

    pub fn reflectivity(mut self, reflectivity: f32) -> Self {
        self.reflectivity = reflectivity;
        self
    }
}

impl From<MaterialConfig> for Material {
    fn from(config: MaterialConfig) -> Self {
        let color = Coloration::from(config.color);
        Material::new(color)
            .albedo(config.albedo)
            .reflectivity(config.reflectivity)
    }
}