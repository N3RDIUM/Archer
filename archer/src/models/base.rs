use crate::ray::Ray;

pub trait Model {
    fn intersect(&self, incoming: Ray) -> f32;
}
