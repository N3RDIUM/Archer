use nalgebra::{Point3, Vector3};

use crate::ray::Ray;
use crate::vectors::{Color, Normal};
use crate::materials::base::Material;

pub struct NormalMaterial {}

impl Material for NormalMaterial {
    fn bounce(&self, incoming: &Ray, hit_point: Point3<f32>, normal: Normal<f32>) -> Ray {
        let _ = (incoming, hit_point, normal);
        return Ray::new_empty();
    }

    fn add_color(
        &self,
        incoming: &Ray,
        hit_point: Point3<f32>,
        normal: Vector3<f32>,
        color: &Color<f32>
    ) -> Point3<f32> {
        let _ = (incoming, hit_point, color);
        let mut normal_color = Color::new(0.0, 0.0, 0.0);
        normal_color.x = 255.0 * 0.5 * (normal.x + 1.0);
        normal_color.y = 255.0 * 0.5 * (normal.y + 1.0);
        normal_color.z = 255.0 * 0.5 * (normal.z + 1.0);
        return normal_color;
    }
}
