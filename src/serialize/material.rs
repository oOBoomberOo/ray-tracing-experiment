use super::*;
#[derive(Deserialize, Clone)]
pub struct MaterialConfig {
    pub color: ColorConfig,
    pub albedo: f32,
    pub reflectivity: f32
}

#[derive(Deserialize, Clone)]
#[serde(untagged)]
pub enum ColorConfig {
    Color([f32; 3]),
    Texture { texture: String }
}