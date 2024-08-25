use bvh::aabb::Aabb;
use nalgebra::{Point3, Vector3};

use crate::ray::Ray;
use crate::vectors::Normal;
use crate::geometries::base::Geometry;

pub struct Sphere {
    pub radius: f32,
    pub position: Point3<f32>,
}

impl Geometry for Sphere {
    fn intersect(&self, incoming: &Ray) -> (nalgebra::Point3<f32>, Vector3<f32>) {
        // Origin
        let oc: Vector3<f32> = self.position - incoming.origin;

        // Coefficients and discriminant
        let a: f32 = incoming.direction.dot(&incoming.direction);
        let b: f32 = -2.0 * incoming.direction.dot(&oc);
        let c: f32 = oc.dot(&oc) - self.radius * self.radius;
        let discriminant: f32 = b * b - 4.0 * a * c;

        if discriminant >= 0.0 {
            let t1 = -b - discriminant.sqrt() / (2.0 * a); // Near
            let _t2 = -b + discriminant.sqrt() / (2.0 * a); // Far

            // TODO: Calculate the normal
            let normal = Normal::new(f32::NAN, f32::NAN, f32::NAN);

            return (incoming.position_at(t1), normal);
        }

        return (
            Point3::new(f32::NAN, f32::NAN, f32::NAN),
            Vector3::new(f32::NAN, f32::NAN, f32::NAN),
        );
    }

    fn aabb(&self) -> Aabb<f32, 3> {
        let half_size = Vector3::new(self.radius, self.radius, self.radius);
        let min = self.position - half_size;
        let max = self.position + half_size;
        Aabb::with_bounds(min, max)
    }
}
