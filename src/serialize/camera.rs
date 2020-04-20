use super::*;

#[derive(Deserialize)]
pub struct CameraConfig {
    pub fov: f32,
    pub width: f32,
    pub height: f32,
    pub speed: f32
}