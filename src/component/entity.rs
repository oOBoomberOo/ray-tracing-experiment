use super::*;
use std::cmp::Ordering;
use std::fmt::Debug;

pub trait Entity: Debug {
    fn material(&self) -> &Material;
    fn intersect(&self, ray: &Ray) -> Option<Intersection>;
    fn surface_normal(&self, contact: Vec3) -> Vec3;
    fn texture_coords(&self, contact: Vec3) -> TextureCoord;
}

pub type Unit = Box<dyn Entity>;
pub type Entities = Vec<Unit>;

pub struct Intersection<'a> {
    entity: &'a dyn Entity,
    relative: Vec3,
    pub source: Ray
}

impl<'a> Intersection<'a> {
    pub fn new(relative: Vec3, entity: &'a dyn Entity, source: Ray) -> Self {
        Intersection { relative, entity, source }
    }

    pub fn fast_dist(&self) -> f32 {
        self.relative.dot(self.relative)
    }

    pub fn texture_coord(&self, contact: Vec3) -> TextureCoord {
        self.entity.texture_coords(contact)
    }

    pub fn color(&self, contact: Vec3) -> Color {
        let texture_coord = self.texture_coord(contact);
        self.entity.material().color.color(&texture_coord)
    }

    pub fn contact(&self) -> Vec3 {
        self.source.origin + self.relative
    }

    pub fn surface_normal(&self) -> Vec3 {
        let contact = self.contact();
        self.entity.surface_normal(contact)
    }

    pub fn reflected_light(&self) -> f32 {
        self.entity.material().albedo / std::f32::consts::PI
    }

    pub fn reflectivity(&self) -> f32 {
        self.entity.material().reflectivity
    }

    pub fn incident(&self) -> Vec3 {
        self.source.direction
    }
}

impl PartialEq for Intersection<'_> {
    fn eq(&self, other: &Intersection) -> bool {
        let this = self.fast_dist();
        let other = other.fast_dist();

        this.eq(&other)
    }
}

impl Eq for Intersection<'_> {}

impl PartialOrd for Intersection<'_> {
    fn partial_cmp(&self, other: &Intersection) -> Option<Ordering> {
        let this = self.fast_dist();
        let other = other.fast_dist();

        this.partial_cmp(&other)
    }
}

impl Ord for Intersection<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
    }
}

pub struct TextureCoord {
    pub x: f32,
    pub y: f32
}