use nalgebra::{Point3, Vector3};

use crate::ray::Ray;
use crate::vectors::{Color, Normal};
use crate::materials::base::Material;

pub struct PerfectMirror {}

impl PerfectMirror {
    pub fn reflect(v: Vector3<f32>, n: Vector3<f32>) -> Vector3<f32> {
        v - 2.0 * v.dot(&n) * n
    }
}

impl Material for PerfectMirror {
    fn bounce(&self, incoming: &Ray, hit_point: Point3<f32>, normal: Normal<f32>) -> Ray {
        let mut bounced = Ray::new_empty();
        bounced.origin = hit_point;
        bounced.direction = PerfectMirror::reflect(incoming.direction, normal);

        return bounced;
    }

    fn add_color(
        &self,
        incoming: &Ray,
        hit_point: Point3<f32>,
        normal: Vector3<f32>,
        color: &Color<f32>
    ) -> Point3<f32> {
        let _ = (incoming, hit_point, normal);
        return *color;
    }
}
