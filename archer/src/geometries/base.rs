use crate::ray::Ray;
use crate::vectors::Vec3;

pub trait Geometry {
    fn intersect(&self, incoming: Ray) -> (Vec3, Vec3);
}

pub struct BaseGeometry {}
impl Geometry for BaseGeometry {
    fn intersect(&self, incoming: Ray) -> (Vec3, Vec3) {
        let _ = incoming;
        return (Vec3::new_empty(), Vec3::new_empty());
    }
}

