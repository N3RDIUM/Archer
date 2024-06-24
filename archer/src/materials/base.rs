use crate::ray::Ray;

pub trait Material {
    fn intersect(&self, incoming: Ray) -> f32;
}
