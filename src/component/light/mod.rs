use super::*;
use crate::scene::{Scene, RECURSION_DEPTH};

mod directional_light;
mod spherical_light;
pub use directional_light::*;
pub use spherical_light::*;

pub trait Light {
	fn intensity(&self) -> f32;
	fn color(&self) -> &Color;
	fn shading(&self, intersection: &Intersection, scene: &Scene) -> Color;

	fn compute_light(&self, intersection: &Intersection, scene: &Scene, depth: usize) -> Color {
		let color = self.shading(intersection, scene);
		let reflection_ray = Ray::create_reflection(intersection);

		color + self.compute_ray(scene, &reflection_ray, depth + 1) * intersection.reflectivity()
	}

	fn compute_ray(&self, scene: &Scene, ray: &Ray, depth: usize) -> Color {
		if depth >= RECURSION_DEPTH {
			return Color::BACKGROUND;
		}

		let intersection = scene.compute_pixel(ray);
		intersection.map_or(Color::BACKGROUND, |x| scene.compute_light(x, depth))
	}
}

pub type LightSource = Box<dyn Light>;
pub type Lights = Vec<LightSource>;
