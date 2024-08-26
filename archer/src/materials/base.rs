use nalgebra::{Point3, Vector3};

use crate::ray::Ray;
use crate::vectors::{Color, Normal};

pub trait Material {
    fn bounce(&self, incoming: &Ray, hit_point: Point3<f64>, normal: Normal<f64>) -> Ray;
    fn add_color(&self, incoming: &Ray, hit_point: Point3<f64>, normal: Normal<f64>, previous_color: &Color<f64>) -> Color<f64>;
}

pub struct BaseMaterial {}
impl Material for BaseMaterial {
    fn bounce(&self, incoming: &Ray, hit_point: Point3<f64>, normal: Normal<f64>) -> Ray {
        let _ = (incoming, hit_point, normal);
        return Ray {
            origin: Point3::new(f64::NAN, f64::NAN, f64::NAN),
            direction: Vector3::new(f64::NAN, f64::NAN, f64::NAN),
        };
    }

    fn add_color(
        &self,
        incoming: &Ray,
        hit_point: Point3<f64>,
        normal: Vector3<f64>,
        color: &Color<f64>
    ) -> Color<f64> {
        let _ = (incoming, hit_point, normal, color);
        return Color::new(0.0, 0.0, 0.0);
    }
}
