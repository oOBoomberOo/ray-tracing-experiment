use crate::serialize::CameraConfig;
use ultraviolet::Vec3;
use winit::{dpi::LogicalSize, event::VirtualKeyCode};
use winit_input_helper::WinitInputHelper;

#[derive(Debug, Default, Clone)]
pub struct Camera {
	pub width: f32,
	pub height: f32,
	pub fov: f32,
	pub aspect_ratio: f32,
	pub pos: Vec3,
	pub speed: f32,
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

	pub fn update(&mut self, input: &WinitInputHelper) {
		let mut movement = Vec3::default();

		if input.key_held(VirtualKeyCode::W) {
			movement -= Vec3::unit_z();
		}
		else if input.key_held(VirtualKeyCode::S) {
			movement += Vec3::unit_z();
		}

		if input.key_held(VirtualKeyCode::A) {
			movement -= Vec3::unit_x();
		}
		else if input.key_held(VirtualKeyCode::D) {
			movement += Vec3::unit_x();
		}

		self.pos += movement * self.speed;
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

	pub fn speed(mut self, speed: f32) -> Self {
		self.speed = speed;
		self
	}
}

impl From<CameraConfig> for Camera {
	fn from(config: CameraConfig) -> Self {
		Camera::new()
			.size(config.width, config.height)
			.fov(config.fov)
			.speed(config.speed)
	}
}
