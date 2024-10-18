use nalgebra::{Point3, Vector3};

use crate::materials::base::Material;
use crate::ray::Ray;
use crate::vectors::{ColorVector, Normal};

pub struct SolidColor {
    pub color: ColorVector<f64>,
}

impl Material for SolidColor {
    fn bounce(&self, _incoming: &Ray, _hit_point: Point3<f64>, _normal: Normal<f64>) -> Ray {
        Ray::new_empty()
    }

    fn add_color(
        &self,
        _incoming: &Ray,
        _hit_point: Point3<f64>,
        _normal: Vector3<f64>,
        _color: &ColorVector<f64>,
    ) -> ColorVector<f64> {
        self.color
    }
}
