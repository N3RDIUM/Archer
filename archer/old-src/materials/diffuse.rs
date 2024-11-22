use nalgebra::{Point3, Vector3};
use rand::prelude::*;

use crate::materials::base::Material;
use crate::ray::Ray;
use crate::vectors::{ColorVector, Normal};

pub struct Diffuse {
    pub color: ColorVector<f64>,
    pub roughness: f64,
    pub albedo: f64,
}

impl Material for Diffuse {
    fn bounce(&self, incoming: &Ray, hit_point: Point3<f64>, normal: Normal<f64>) -> Ray {
        let mut rng = rand::thread_rng();

        let v = incoming.direction.normalize();
        let n = normal.normalize();

        // Generate a random vector in the range [-1, 1)
        let r: Vector3<f64> =
            Vector3::new(rng.gen(), rng.gen(), rng.gen()) * 2.0 - Vector3::new(1.0, 1.0, 1.0);

        // Calculate the bounced ray
        let direction =
            ((v - 2.0 * n.dot(&v) * n) * (1.0 - self.roughness) + r * self.roughness) * 0.5;

        Ray {
            origin: hit_point,
            direction,
        }
    }

    fn add_color(
        &self,
        _incoming: &Ray,
        _hit_point: Point3<f64>,
        _normal: Vector3<f64>,
        previous_color: &ColorVector<f64>,
    ) -> ColorVector<f64> {
        ColorVector::new(
            (self.color.x * (1.0 - self.albedo) + previous_color.x * self.albedo) / 2.0,
            (self.color.y * (1.0 - self.albedo) + previous_color.y * self.albedo) / 2.0,
            (self.color.z * (1.0 - self.albedo) + previous_color.z * self.albedo) / 2.0,
        )
    }
}
