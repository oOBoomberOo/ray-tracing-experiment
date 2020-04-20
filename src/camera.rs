use crate::serialize::CameraConfig;
use winit::dpi::LogicalSize;

#[derive(Debug, Default, Clone)]
pub struct Camera {
    pub width: f32,
    pub height: f32,
    pub fov: f32,
    pub aspect_ratio: f32,
}

impl Camera {
    pub fn new() -> Camera {
        Camera::default()
    }

    pub fn screen(&self) -> (f32, f32) {
        (self.width, self.height)
    }

    pub fn get_size(&self) -> LogicalSize<f32> {
        LogicalSize::new(self.width, self.height)
    }
}

impl Camera {
    pub fn size(mut self, width: f32, height: f32) -> Self {
        self.width = width;
        self.height = height;
        self.aspect_ratio = width / height;
        self
    }
    pub fn fov(mut self, fov: f32) -> Self {
        self.fov = (fov.to_radians() * 0.5).tan();
        self
    }
}

impl From<CameraConfig> for Camera {
    fn from(config: CameraConfig) -> Self {
        Camera::new()
            .size(config.width, config.height)
            .fov(config.fov)
    }
}
