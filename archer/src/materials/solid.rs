use nalgebra::{Point3, Vector3};

use crate::ray::Ray;
use crate::vectors::{Color, Normal};
use crate::materials::base::Material;

pub struct SolidColor {
    pub color: Color<f32>,
}

impl Material for SolidColor {
    fn bounce(&self, incoming: &Ray, hit_point: Point3<f32>, normal: Normal<f32>) -> Ray {
        let _ = (incoming, hit_point, normal);
        return Ray::new_empty();
    }

    fn add_color(
        &self,
        incoming: &Ray,
        hit_point: Point3<f32>,
        normal: Vector3<f32>,
    ) -> Point3<f32> {
        let _ = (incoming, hit_point, normal);
        return self.color;
    }
}
