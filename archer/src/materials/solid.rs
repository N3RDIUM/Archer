use crate::materials::base::Material;
use crate::vectors::Vec3;
use crate::ray::Ray;

pub struct SolidColor {
    pub color: [u8; 3]
}

impl Material for SolidColor {
    fn bounce(&self, incoming: Ray, hit_point: Vec3, normal: Vec3) -> Ray {
        let _ = (incoming, hit_point, normal);
        return Ray::new_empty()
    }

    fn add_color(&self, incoming: Ray, hit_point: Vec3, normal: Vec3) -> [u8; 3] {
        let _ = (incoming, hit_point, normal);
        return self.color;
    }
}

