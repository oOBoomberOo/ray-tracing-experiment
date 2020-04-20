use super::*;
use crate::camera::Camera;
use std::ops::Range;
use ultraviolet::Vec2;

#[derive(Debug, Clone)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    const BIAS: f32 = 1e-3;

    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray { origin, direction }
    }

    pub fn create_prime(pixel: Vec2, camera: &Camera) -> Ray {
        let (width, height) = camera.screen();
        let Vec2 { x, y } = pixel;

        let x = (x + 0.5).map_range(0.0..width, -1.0..1.0);
        let y = (y + 0.5).map_range(0.0..height, -1.0..1.0);

        let x = x * camera.aspect_ratio * camera.fov;
        let y = y * camera.fov;

        let origin = Vec3::zero();
        let direction = Vec3::new(x, y, -1.0).normalized();

        Ray::new(origin, direction)
    }

    pub fn create_reflection(intersection: &Intersection) -> Ray {
        let normal = intersection.surface_normal();
        let incident = intersection.incident();
        let origin = intersection.contact() + (normal * Ray::BIAS);
        let direction = incident - (2.0 * incident.dot(normal) * normal);
        Ray::new(origin, direction)
    }
}

trait MapRange
where
    Self: Sized,
{
    fn map_range(&self, from: Range<Self>, to: Range<Self>) -> Self;
}

impl MapRange for f32 {
    fn map_range(&self, from: Range<Self>, to: Range<Self>) -> Self {
        (self - from.start) / (from.end - from.start) * (to.end - to.start) + to.start
    }
}
