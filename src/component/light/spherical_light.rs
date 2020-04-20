use super::*;
use crate::scene::SHADOW_BIAS;

#[derive(Debug, Default, Clone)]
pub struct SphericalLight {
	pos: Vec3,
	color: Color,
	intensity: f32,
}

impl SphericalLight {
	pub fn new() -> SphericalLight {
		SphericalLight::default()
	}

	pub fn distance(&self, contact: Vec3) -> f32 {
		let diff = self.pos - contact;
		diff.dot(diff)
	}
}

impl SphericalLight {
	pub fn pos(mut self, pos: Vec3) -> Self {
		self.pos = pos;
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

impl Light for SphericalLight {
	fn intensity(&self) -> f32 {
		self.intensity
	}
	fn color(&self) -> &Color {
		&self.color
	}

	fn shading(&self, intersection: &Intersection, scene: &Scene) -> Color {
		use std::f32::consts::PI;

		let surface_normal = intersection.surface_normal();
		let contact = intersection.contact();
		let direction_to_light = (self.pos - contact).normalized();
		let decay = 4.0 * PI * (self.pos - contact).mag_sq();

		let contact = intersection.contact();
		let origin = contact + (surface_normal * SHADOW_BIAS);
		let shadow_ray = Ray::new(origin, direction_to_light);

		let is_shadow = scene.compute_pixel(&shadow_ray).map_or(false, |intersect| {
			intersect.fast_dist() > self.distance(contact)
		});
		let shadow_intensity = if is_shadow { 0.1 } else { 1.0 };

		let light_power =
			surface_normal.dot(direction_to_light).max(0.0) * self.intensity * shadow_intensity
				/ decay;
		let light_reflected = intersection.reflected_light();

		intersection.color(contact) * self.color * light_power * light_reflected
	}
}
