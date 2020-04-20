use super::*;
use crate::component::*;
use anyhow::Result;
use serde_json::Value;

#[derive(Deserialize)]
pub struct EntityConfig {
	r#type: String,
	data: Value,
}

impl EntityConfig {
	pub fn compile(self) -> Result<Unit> {
		let result = match self.r#type.as_ref() {
			"sphere" => {
				let config: SphereConfig = serde_json::from_value(self.data)?;
				config.compile()
			}
			"plane" => {
				let config: PlaneConfig = serde_json::from_value(self.data)?;
				config.compile()
			}
			_ => panic!("Invalid entity type"),
		};

		Ok(result)
	}
}

#[derive(Deserialize)]
struct SphereConfig {
	pos: [f32; 3],
	radius: f32,
	material: MaterialConfig,
}

impl SphereConfig {
	pub fn compile(self) -> Unit {
		let pos = Vec3::from(self.pos);
		let radius = self.radius;
		let material_config = self.material;
		let material = Material::from(material_config);
		let result = Sphere::new().pos(pos).radius(radius).material(material);
		Box::new(result)
	}
}

#[derive(Deserialize)]
struct PlaneConfig {
	pos: [f32; 3],
	normal: [f32; 3],
	material: MaterialConfig,
}

impl PlaneConfig {
	pub fn compile(self) -> Unit {
		let pos = Vec3::from(self.pos);
		let normal = Vec3::from(self.normal);
		let material = Material::from(self.material);
		let result = Plane::new().pos(pos, normal).material(material);
		Box::new(result)
	}
}
