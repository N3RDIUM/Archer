use crate::ray::Ray;

pub trait Geometry {
    fn intersect(&self, incoming: Ray) -> f32;
}
