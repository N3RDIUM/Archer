use nalgebra::{Point3, Vector3};

use crate::ray::Ray;
use crate::vectors::{Color, Normal};

pub trait Material {
    fn bounce(&self, incoming: &Ray, hit_point: Point3<f32>, normal: Normal<f32>) -> Ray;
    fn add_color(&self, incoming: &Ray, hit_point: Point3<f32>, normal: Normal<f32>, previous_color: &Color<f32>) -> Color<f32>;
}

pub struct BaseMaterial {}
impl Material for BaseMaterial {
    fn bounce(&self, incoming: &Ray, hit_point: Point3<f32>, normal: Normal<f32>) -> Ray {
        let _ = (incoming, hit_point, normal);
        return Ray {
            origin: Point3::new(f32::NAN, f32::NAN, f32::NAN),
            direction: Vector3::new(f32::NAN, f32::NAN, f32::NAN),
        };
    }

    fn add_color(
        &self,
        incoming: &Ray,
        hit_point: Point3<f32>,
        normal: Vector3<f32>,
        color: &Color<f32>
    ) -> Color<f32> {
        let _ = (incoming, hit_point, normal, color);
        return Color::new(0.0, 0.0, 0.0);
    }
}
