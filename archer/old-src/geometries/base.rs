use bvh::aabb::Aabb;
use nalgebra::{Point3, Vector3};

use crate::ray::Ray;

pub trait Geometry {
    fn intersect(&self, incoming: &Ray) -> Option<(Point3<f64>, Vector3<f64>)>;
    fn aabb(&self) -> Aabb<f64, 3>;
}

// Dummy geometry that does nothing
pub struct BaseGeometry;

impl Geometry for BaseGeometry {
    fn intersect(&self, _incoming: &Ray) -> Option<(Point3<f64>, Vector3<f64>)> {
        None
    }

    fn aabb(&self) -> Aabb<f64, 3> {
        // Define a random AABB for the dummy geometry
        let half_size = Vector3::new(0.1, 0.0, -0.1);
        let position = Point3::new(0.1, 0.2, 0.3);

        // Create AABB based on the defined position and half_size
        let min = position - half_size;
        let max = position + half_size;

        Aabb::with_bounds(min, max)
    }
}
