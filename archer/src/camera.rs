use crate::ray::Ray;
use crate::vectors::Vec2;
use crate::vectors::Vec3;

#[derive(Debug)]
pub struct Camera {
    pub resolution: Vec2,
    pub focal_length: f64,
    pub viewport_height: f64,
    pub position: Vec3,
    pub rotation: Vec3,

    // These things are set by the `update` func!
    aspect: f64,
    viewport_width: f64,
    viewport_u: Vec3,
    viewport_v: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    top_left: Vec3,
    top_left_location: Vec3,
}

impl Camera {
    pub fn new(resolution: [u32; 2]) -> Camera {
        return Camera {
            resolution: Vec2 {
                x: resolution[0] as f64,
                y: resolution[1] as f64,
            },
            focal_length: 1.0,
            viewport_height: 2.0,
            position: Vec3::fill(0.0),
            rotation: Vec3::fill(0.0),

            aspect: 0.0,
            viewport_width: 1.0,
            viewport_u: Vec3::fill(0.0),
            viewport_v: Vec3::fill(0.0),
            pixel_delta_u: Vec3::fill(0.0),
            pixel_delta_v: Vec3::fill(0.0),
            top_left: Vec3::fill(0.0),
            top_left_location: Vec3::fill(0.0),
        };
    }

    pub fn update(&mut self) {
        self.aspect = self.resolution.x / self.resolution.y;
        self.viewport_width = self.viewport_height * self.aspect;

        // Calculate viewport stuff
        self.viewport_u = Vec3 {
            x: self.viewport_width,
            y: 0.0,
            z: 0.0,
        };
        self.viewport_v = Vec3 {
            y: -self.viewport_height,
            x: 0.0,
            z: 0.0,
        };

        // Calculate pixel deltas
        self.pixel_delta_u = self.viewport_u / Vec3::fill(self.resolution.x);
        self.pixel_delta_v = self.viewport_v / Vec3::fill(self.resolution.y);

        // Calculate the location of the upper left pixel
        let focal_length: Vec3 = Vec3 {
            x: 0.0,
            y: 0.0,
            z: self.focal_length,
        };
        self.top_left = self.position
            - focal_length
            - self.viewport_u / Vec3::fill(2 as f64)
            - self.viewport_v / Vec3::fill(2 as f64);
        self.top_left_location =
            self.top_left + Vec3::fill(0.5) * (self.pixel_delta_u + self.pixel_delta_v);
    }

    pub fn get_ray(&self, pixel: Vec2) -> Ray {
        let pixel_center: Vec3 = self.top_left_location
            + Vec3::fill(pixel.x) * self.pixel_delta_u
            + Vec3::fill(pixel.y) * self.pixel_delta_v;
        let ray_direction = pixel_center - self.position;

        return Ray {
            origin: self.position,
            direction: ray_direction,
        };
    }
}
