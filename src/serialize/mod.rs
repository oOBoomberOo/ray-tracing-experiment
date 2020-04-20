mod camera;
mod entity;
mod light;
mod material;

pub use camera::*;
pub use entity::*;
pub use light::*;
pub use material::*;
pub use serde::Deserialize;

use anyhow::Result;

#[derive(Deserialize)]
pub struct Config {
	entities: Vec<EntityConfig>,
	lights: Vec<LightConfig>,
	camera: CameraConfig,
}

use crate::{
	camera::Camera,
	component::{Entities, Lights},
};
impl Config {
	pub fn compile(self) -> Result<(Entities, Lights, Camera)> {
		let entities: Result<_> = self
			.entities
			.into_iter()
			.map(EntityConfig::compile)
			.collect();
		let lights: Result<_> = self.lights.into_iter().map(LightConfig::compile).collect();

		let entities = entities?;
		let lights = lights?;
		let camera = Camera::from(self.camera);
		let result = (entities, lights, camera);
		Ok(result)
	}
}
