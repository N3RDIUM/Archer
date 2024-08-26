use bvh::aabb::Aabb;
use nalgebra::{Point3, Vector3};

use crate::ray::Ray;

pub trait Geometry {
    fn intersect(&self, incoming: &Ray) -> Option<(nalgebra::Point3<f64>, Vector3<f64>)>;
    fn aabb(&self) -> Aabb<f64, 3>;
}

pub struct BaseGeometry {}
impl Geometry for BaseGeometry {
    fn intersect(&self, incoming: &Ray) -> Option<(nalgebra::Point3<f64>, Vector3<f64>)> {
        let _ = incoming;
        return None;
    }

    fn aabb(&self) -> Aabb<f64, 3> {
        // Just some randon stuff for now
        let half_size = Vector3::new(0.1, 0.0, -0.1);
        let position = Point3::new(0.1, 0.2, 0.3);

        // Sphere stuff bcoz i had nothing else to put here
        let min = position - half_size;
        let max = position + half_size;

        return Aabb::with_bounds(min, max);
    }
}
