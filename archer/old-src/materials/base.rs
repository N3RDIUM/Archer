use nalgebra::{Point3, Vector3};

use crate::ray::Ray;
use crate::vectors::{ColorVector, Normal};

pub trait Material {
    fn bounce(&self, incoming: &Ray, hit_point: Point3<f64>, normal: Normal<f64>) -> Ray;
    fn add_color(
        &self,
        incoming: &Ray,
        hit_point: Point3<f64>,
        normal: Normal<f64>,
        previous_color: &ColorVector<f64>,
    ) -> ColorVector<f64>;
}

// Dummy material that does nothing
pub struct BaseMaterial;

impl Material for BaseMaterial {
    fn bounce(&self, _incoming: &Ray, _hit_point: Point3<f64>, _normal: Normal<f64>) -> Ray {
        Ray {
            origin: Point3::new(f64::NAN, f64::NAN, f64::NAN),
            direction: Vector3::new(f64::NAN, f64::NAN, f64::NAN),
        }
    }

    fn add_color(
        &self,
        _incoming: &Ray,
        _hit_point: Point3<f64>,
        _normal: Normal<f64>,
        _previous_color: &ColorVector<f64>,
    ) -> ColorVector<f64> {
        ColorVector::new(0.0, 0.0, 0.0)
    }
}
