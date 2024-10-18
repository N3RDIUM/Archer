use bvh::aabb::Aabb;
use nalgebra::{Point3, Vector3};

use crate::geometries::base::Geometry;
use crate::ray::Ray;

pub struct Sphere {
    pub radius: f64,
    pub position: Point3<f64>,
}

impl Geometry for Sphere {
    fn intersect(&self, incoming: &Ray) -> Option<(Point3<f64>, Vector3<f64>)> {
        // Origin
        let oc = incoming.origin - self.position;

        // Coefficients
        let a = incoming.direction.norm_squared();
        let b = 2.0 * incoming.direction.dot(&oc);
        let c = oc.norm_squared() - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            return None;
        }

        // Calculate the nearest intersection point
        let t = (-b - discriminant.sqrt()) / (2.0 * a);

        // Shadow Acne fix
        if t < f64::EPSILON {
            return None;
        }

        let point = incoming.position_at(t);
        let normal = (point - self.position).normalize();

        Some((point, normal))
    }

    fn aabb(&self) -> Aabb<f64, 3> {
        let half_size = Vector3::new(self.radius, self.radius, self.radius);
        let min = self.position - half_size;
        let max = self.position + half_size;
        Aabb::with_bounds(min, max)
    }
}
