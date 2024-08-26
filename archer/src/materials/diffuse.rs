use rand::prelude::*;
use nalgebra::{Point3, Vector3};

use crate::ray::Ray;
use crate::vectors::{Color, Normal};
use crate::materials::base::Material;

pub struct Diffuse {
    pub color: Color<f64>,
    pub roughness: f64,
    pub albedo: f64
}

impl Material for Diffuse {
    fn bounce(&self, incoming: &Ray, hit_point: Point3<f64>, normal: Normal<f64>) -> Ray {
        let mut bounced = Ray::new_empty();
        let mut rng = rand::thread_rng();

        let v = incoming.direction.normalize();
        let n = normal.normalize();
        let mut r: Vector3<f64> = Vector3::new(
            rng.gen(), rng.gen(), rng.gen()
        ) * 2.0;
        r -= Vector3::new(1.0, 1.0, 1.0);

        bounced.origin = hit_point;
        bounced.direction = ((v - 2.0 * n.dot(&v) * n) * (1.0 - self.roughness) + r * self.roughness) * 0.5;

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
        let ret = Color::new(
            (self.color.x * (1.0 - self.albedo) + color.x * self.albedo) / 2.0,
            (self.color.y * (1.0 - self.albedo) + color.y * self.albedo) / 2.0,
            (self.color.z * (1.0 - self.albedo) + color.z * self.albedo) / 2.0,
        );
        return ret;
    }
}
