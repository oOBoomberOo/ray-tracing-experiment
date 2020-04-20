use super::*;
use crate::component::*;
use anyhow::Result;
use serde_json::Value;

#[derive(Deserialize)]
pub struct LightConfig {
	r#type: String,
	data: Value,
}

impl LightConfig {
	pub fn compile(self) -> Result<LightSource> {
		let result = match self.r#type.as_ref() {
			"directional_light" => {
				let config: DirectionalLightConfig = serde_json::from_value(self.data)?;
				config.compile()
			}
			"spherical_light" => {
				let config: SphericalLightConfig = serde_json::from_value(self.data)?;
				config.compile()
			}
			_ => panic!("Invalid entity type"),
		};

		Ok(result)
	}
}

#[derive(Deserialize)]
struct DirectionalLightConfig {
	direction: [f32; 3],
	color: [f32; 3],
	intensity: f32,
}

impl DirectionalLightConfig {
	pub fn compile(self) -> LightSource {
		let direction = Vec3::from(self.direction);
		let color = Color::from(self.color);
		let intensity = self.intensity;

		let result = DirectionalLight::new()
			.direction(direction)
			.color(color)
			.intensity(intensity);

		Box::new(result)
	}
}

#[derive(Deserialize)]
struct SphericalLightConfig {
	pos: [f32; 3],
	color: [f32; 3],
	intensity: f32,
}

impl SphericalLightConfig {
	pub fn compile(self) -> LightSource {
		let pos = Vec3::from(self.pos);
		let color = Color::from(self.color);
		let intensity = self.intensity;

		let result = SphericalLight::new()
			.pos(pos)
			.color(color)
			.intensity(intensity);

		Box::new(result)
	}
}
