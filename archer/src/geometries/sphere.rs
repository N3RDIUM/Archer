use bvh::aabb::Aabb;
use nalgebra::{Point3, Vector3};

use crate::ray::Ray;
use crate::geometries::base::Geometry;

pub struct Sphere {
    pub radius: f64,
    pub position: Point3<f64>,
}

impl Geometry for Sphere {
    fn intersect(&self, incoming: &Ray) -> Option<(Point3<f64>, Vector3<f64>)> {
        // Origin
        let oc: Vector3<f64> = incoming.origin - self.position;

        // Coefficients and discriminant
        let a: f64 = incoming.direction.dot(&incoming.direction);
        let b: f64 = 2.0 * incoming.direction.dot(&oc);
        let c: f64 = oc.dot(&oc) - self.radius * self.radius;
        let discriminant: f64 = b * b - 4.0 * a * c;

        if discriminant >= 0.0 {
            let t = (-b - f64::sqrt(discriminant)) / (2.0 * a);

            if t < 0.00000000000000001 { return None }

            let point = incoming.position_at(t);
            let normal = (point - self.position).normalize();

            return Some((point, normal));
        }

        return None;
    }

    fn aabb(&self) -> Aabb<f64, 3> {
        let half_size = Vector3::new(self.radius, self.radius, self.radius);
        let min = self.position - half_size;
        let max = self.position + half_size;
        Aabb::with_bounds(min, max)
    }
}
