use super::*;
use crate::scene::SHADOW_BIAS;

#[derive(Debug, Default, Clone)]
pub struct DirectionalLight {
	direction: Vec3,
	color: Color,
	intensity: f32,
}

impl DirectionalLight {
	pub fn new() -> DirectionalLight {
		DirectionalLight::default()
	}
}

impl DirectionalLight {
	pub fn direction(mut self, direction: Vec3) -> Self {
		self.direction = direction.normalized();
		self
	}

	pub fn color(mut self, color: Color) -> Self {
		self.color = color.clamp();
		self
	}

	pub fn intensity(mut self, intensity: f32) -> Self {
		self.intensity = intensity;
		self
	}
}

impl Light for DirectionalLight {
	fn intensity(&self) -> f32 {
		self.intensity
	}
	fn color(&self) -> &Color {
		&self.color
	}
	fn shading(&self, intersection: &Intersection, scene: &Scene) -> Color {
		let surface_normal = intersection.surface_normal();
		let direction_to_light = -self.direction;
		let contact = intersection.contact();
		let origin = contact + (surface_normal * SHADOW_BIAS);
		let shadow_ray = Ray::new(origin, direction_to_light);

		let is_shadow = scene.compute_pixel(&shadow_ray).is_some();
		let shadow_intensity = if is_shadow { 0.0 } else { 1.0 };

		let light_power = surface_normal.dot(direction_to_light).max(0.0) * self.intensity * shadow_intensity;
		let light_reflected = intersection.reflected_light();

		intersection.color(contact) * self.color * light_power * light_reflected
	}
}
