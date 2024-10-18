use nalgebra::{Point3, Vector3};

use crate::materials::base::Material;
use crate::ray::Ray;
use crate::vectors::{ColorVector, Normal};

pub struct NormalMaterial;

impl Material for NormalMaterial {
    fn bounce(&self, _incoming: &Ray, _hit_point: Point3<f64>, _normal: Normal<f64>) -> Ray {
        Ray::new_empty()
    }

    fn add_color(
        &self,
        _incoming: &Ray,
        _hit_point: Point3<f64>,
        normal: Vector3<f64>,
        _color: &ColorVector<f64>,
    ) -> ColorVector<f64> {
        ColorVector::new(
            0.5 * (normal.x + 1.0) * 255.0,
            0.5 * (normal.y + 1.0) * 255.0,
            0.5 * (normal.z + 1.0) * 255.0,
        )
    }
}
