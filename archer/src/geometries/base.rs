use crate::ray::Ray;
use crate::vectors::Vec3;

pub trait Geometry {
    fn intersect(&self, incoming: Ray) -> Vec3;
}
