// TODO: PORT EVERYTHING TO NALGEBRA
use crate::ray::Ray;
use crate::vectors::Normal;
use nalgebra::Point3;

pub trait Geometry {
    fn intersect(&self, incoming: Ray) -> (Point3<f32>, Normal<f32>); /* Hit point, Normal */
}

pub struct BaseGeometry {}
impl Geometry for BaseGeometry {
    fn intersect(&self, incoming: Ray) -> (Point3<f32>, Normal<f32>) {
        let _ = incoming;
        return (
            Point3::new(f32::NAN, f32::NAN, f32::NAN),
            Normal::new(f32::NAN, f32::NAN, f32::NAN),
        );
    }
}
