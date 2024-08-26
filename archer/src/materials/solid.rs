use nalgebra::{Point3, Vector3};

use crate::ray::Ray;
use crate::vectors::{Color, Normal};
use crate::materials::base::Material;

pub struct SolidColor {
    pub color: Color<f64>,
}

impl Material for SolidColor {
    fn bounce(&self, incoming: &Ray, hit_point: Point3<f64>, normal: Normal<f64>) -> Ray {
        let _ = (incoming, hit_point, normal);
        return Ray::new_empty();
    }

    fn add_color(
        &self,
        incoming: &Ray,
        hit_point: Point3<f64>,
        normal: Vector3<f64>,
        color: &Color<f64>
    ) -> Point3<f64> {
        let _ = (incoming, hit_point, normal, color);
        return self.color;
    }
}
