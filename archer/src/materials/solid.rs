use crate::materials::base::Material;
use crate::vectors::Vec3;
use crate::ray::Ray;

pub struct SolidColor {
    pub color: Vec3
}

impl Material for SolidColor {
    fn bounce(&self, incoming: Ray, hit_point: Vec3, normal: Vec3) -> Ray {
        let _ = (incoming, hit_point, normal);
        return Ray::new_empty()
    }
}

