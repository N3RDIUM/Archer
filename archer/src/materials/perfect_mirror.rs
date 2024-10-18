use nalgebra::{Point3, Vector3};

use crate::materials::base::Material;
use crate::ray::Ray;
use crate::vectors::{Color, Normal};

pub struct PerfectMirror;

impl Material for PerfectMirror {
    fn bounce(&self, incoming: &Ray, hit_point: Point3<f64>, normal: Normal<f64>) -> Ray {
        let v = incoming.direction.normalize();
        let n = normal.normalize();

        Ray {
            origin: hit_point,
            direction: v - 2.0 * n.dot(&v) * n,
        }
    }

    fn add_color(
        &self,
        _incoming: &Ray,
        _hit_point: Point3<f64>,
        _normal: Vector3<f64>,
        color: &Color<f64>,
    ) -> Color<f64> {
        *color
    }
}
