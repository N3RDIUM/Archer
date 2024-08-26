use nalgebra::{Point3, Vector3};

use crate::ray::Ray;
use crate::vectors::{Color, Normal};
use crate::materials::base::Material;

pub struct PerfectMirror {}

impl Material for PerfectMirror {
    fn bounce(&self, incoming: &Ray, hit_point: Point3<f64>, normal: Normal<f64>) -> Ray {
        let mut bounced = Ray::new_empty();

        let v = incoming.direction.normalize();
        let n = normal.normalize();

        bounced.origin = hit_point;
        bounced.direction = v - 2.0 * n.dot(&v) * n;

        return bounced;
    }

    fn add_color(
        &self,
        incoming: &Ray,
        hit_point: Point3<f64>,
        normal: Vector3<f64>,
        color: &Color<f64>
    ) -> Point3<f64> {
        let _ = (incoming, hit_point, normal);
        return *color;
    }
}
