use super::*;

#[derive(Debug, Default, Clone)]
pub struct Plane {
    pos: Vec3,
    normal: Vec3,
    material: Material
}

impl Plane {
    pub fn new() -> Self {
        Plane::default()
    }
}

impl Plane {
    pub fn pos(mut self, pos: Vec3, normal: Vec3) -> Self {
        self.pos = pos;
        self.normal = normal;
        self
    }

    pub fn material(mut self, material: Material) -> Self {
        self.material = material;
        self
    }
}

impl Entity for Plane {
    fn material(&self) -> &Material {
        &self.material
    }

    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
		let normal = self.normal;
		let denom = normal.dot(ray.direction);

		if denom > 1e-6 {
			let v = self.pos - ray.origin;
			let distance = v.dot(normal) / denom;

			if distance >= 0.0 {
				let relative = ray.direction * distance;
				let entity: &dyn Entity = self;
				let source = ray.clone();
				let result = Intersection::new(relative, entity, source);

				return Some(result)
			}
		}

		None
    }

    fn surface_normal(&self, _: Vec3) -> Vec3 {
        -self.normal
    }

    fn texture_coords(&self, contact: Vec3) -> TextureCoord {
        let mut x_axis = self.normal.cross(Vec3::unit_z());

        if x_axis.mag_sq() == 0.0 {
            x_axis = self.normal.cross(Vec3::unit_y());
        }

        let y_axis = self.normal.cross(x_axis);

        let relative = contact - self.pos;
        let x = relative.dot(x_axis);
        let y = relative.dot(y_axis);

        TextureCoord { x, y }
    }
}