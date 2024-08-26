use crate::ray::Ray;
use rand::prelude::*;
use crate::vectors::PixelCoord;
use nalgebra::{Point3, Vector3};

#[derive(Debug)]
pub struct Camera {
    pub resolution: PixelCoord<f64>,
    pub focal_length: f64,
    pub viewport_height: f64,
    pub position: Point3<f64>,
    pub rotation: Vector3<f64>,
    pub randomness: f64,

    // These things are set by the `update` func!
    pixel_delta_u: Vector3<f64>,
    pixel_delta_v: Vector3<f64>,
    top_left_location: Vector3<f64>,
}

impl Camera {
    pub fn new(resolution: PixelCoord<u32>) -> Camera {
        return Camera {
            resolution: PixelCoord::new(resolution[0] as f64, resolution[1] as f64),
            focal_length: 2.0,
            viewport_height: 2.0,
            position: Point3::new(0.0, 0.0, 0.0),
            rotation: Vector3::new(0.0, 0.0, 0.0),
            randomness: 0.001,

            pixel_delta_u: Vector3::new(0.0, 0.0, 0.0),
            pixel_delta_v: Vector3::new(0.0, 0.0, 0.0),
            top_left_location: Vector3::new(0.0, 0.0, 0.0),
        };
    }

    pub fn update(&mut self) {
        let aspect: f64 = self.resolution.x / self.resolution.y;
        let viewport_width: f64 = self.viewport_height * aspect;

        // Calculate viewport stuff
        let viewport_u: Vector3<f64> = Vector3::new(viewport_width, 0.0, 0.0);
        let viewport_v: Vector3<f64> = Vector3::new(0.0, -self.viewport_height, 0.0);

        // Calculate pixel deltas
        self.pixel_delta_u = viewport_u / self.resolution.x;
        self.pixel_delta_v = viewport_v / self.resolution.y;

        // Calculate the location of the upper left pixel
        let top_left: Vector3<f64> = Vector3::new(
            self.position.x,
            self.position.y,
            self.position.z - self.focal_length,
        ) - viewport_u / 2.0
            - viewport_v / 2.0;
        self.top_left_location = top_left + (self.pixel_delta_u + self.pixel_delta_v) / 2.0;
    }

    pub fn get_ray(&self, pixel: PixelCoord<u32>) -> Ray {
        let pixel_center: Vector3<f64> = self.top_left_location
            + self.pixel_delta_u * pixel.x as f64
            + self.pixel_delta_v * pixel.y as f64;

        let mut rng = rand::thread_rng();
        let ray_direction = pixel_center
            - Vector3::new(self.position.x, self.position.y, self.position.z)
            + Vector3::new(rng.gen(), rng.gen(), rng.gen()) * self.randomness;

        return Ray {
            origin: self.position,
            direction: ray_direction,
        };
    }
}
