use crate::geometries::base::Geometry;
use crate::vectors::Vec3;
use crate::vectors::dot;
use crate::ray::Ray;

pub struct Sphere {
    pub radius: f32,
    pub position: Vec3
}

impl Geometry for Sphere {
    fn intersect(&self, incoming: Ray) -> f32 {
        let oc: Vec3 = self.position - incoming.origin;
        let a: f32 = dot(incoming.direction, incoming.direction);
        let b: f32 = -2.0 * dot(incoming.direction, oc);
        let c: f32 = dot(oc, oc) - self.radius * self.radius;
        let discriminant: f32 = b*b - 4.0*a*c;
        return discriminant;
    }
}

