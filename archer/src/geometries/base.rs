use crate::ray::Ray;
use crate::vectors::Normal;
use bvh::aabb::Aabb;
use nalgebra::{Point3, Vector3};

pub trait Geometry {
    fn intersect(&self, incoming: &Ray) -> (Point3<f32>, Normal<f32>);
    fn aabb(&self) -> Aabb<f32, 3>;
}

pub struct BaseGeometry {}
impl Geometry for BaseGeometry {
    fn intersect(&self, incoming: &Ray) -> (Point3<f32>, Normal<f32>) {
        let _ = incoming;
        return (
            Point3::new(f32::NAN, f32::NAN, f32::NAN),
            Normal::new(f32::NAN, f32::NAN, f32::NAN),
        );
    }

    fn aabb(&self) -> Aabb<f32, 3> {
        // Just some randon stuff for now
        let half_size = Vector3::new(0.1, 0.0, -0.1);
        let position = Point3::new(0.1, 0.2, 0.3);

        // Sphere stuff bcoz i had nothing else to put here
        let min = position - half_size;
        let max = position + half_size;

        return Aabb::with_bounds(min, max);
    }
}
