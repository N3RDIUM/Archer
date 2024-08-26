use nalgebra::{Point3, Vector3};

use crate::ray::Ray;
use crate::vectors::{Color, Normal};
use crate::materials::base::Material;

pub struct NormalMaterial {}

impl Material for NormalMaterial {
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
        let _ = (incoming, hit_point, color);
        return Color::new(
            0.5 * (normal.x + 1.0) * 255.0, 
            0.5 * (normal.y + 1.0) * 255.0, 
            0.5 * (normal.z + 1.0) * 255.0, 
        );
    }
}
