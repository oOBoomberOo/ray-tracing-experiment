use super::*;

#[derive(Debug, Default, Clone)]
pub struct Sphere {
    pos: Vec3,
    radius: f32,
    material: Material
}

impl Sphere {
    pub fn new() -> Self {
        Sphere::default()
    }
}

impl Sphere {
    pub fn pos(mut self, pos: Vec3) -> Self {
        self.pos = pos;
        self
    }

    pub fn radius(mut self, radius: f32) -> Self {
        self.radius = radius;
        self
    }

    pub fn material(mut self, material: Material) -> Self {
        self.material = material;
        self
    }
}

impl Entity for Sphere {
    fn material(&self) -> &Material {
        &self.material
    }

    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let local: Vec3 = self.pos - ray.origin;
        let projected = local.dot(ray.direction);
        let distance = local.dot(local) - projected * projected;
        let radius = self.radius * self.radius;
        
        if distance > radius {
            return None;
        }

        let thicc = (radius - distance).sqrt();
        let edge_front = projected - thicc;
        let edge_back = projected + thicc;

        if edge_front < 0.0 && edge_back < 0.0 {
            return None;
        }

        let distance_to_edge = edge_front.min(edge_back);
        let pos = ray.direction * distance_to_edge;
        let entity: &dyn Entity = self;
        let source = ray.clone();
        let result = Intersection::new(pos, entity, source);
        Some(result)
    }

    fn surface_normal(&self, contact: Vec3) -> Vec3 {
        (contact - self.pos).normalized()
    }

    fn texture_coords(&self, contact: Vec3) -> TextureCoord {
        use std::f32::consts::{FRAC_PI_2, PI};

        let relative = contact - self.pos;
        let x = (1.0 + relative.z.atan2(relative.x)) / FRAC_PI_2;
        let y = (relative.y / self.radius).acos() / PI;
        
        TextureCoord { x, y }
    }
}