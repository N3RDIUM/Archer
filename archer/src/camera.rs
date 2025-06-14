use crate::types::{Position, Direction, PixelCoord};
use crate::ray::Ray;

pub struct Camera {
    pub resolution: PixelCoord<u16>,
    pub focal_length: f64,
    pub viewport_height: f64,
    pub position: Position<f64>,
    pub rotation: Direction<f64>,
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            resolution: PixelCoord::<u16>::origin(),
            focal_length: 2.0, 
            viewport_height: 2.0,
            position: Position::<f64>::origin(),
            rotation: Direction::<f64>::zeros()
        }
    }

    pub fn get_ray(&self, pixel: PixelCoord<u16>) -> Ray {
        // TODO: this should only be computed once unless parameters change
        let aspect: f64 = self.resolution.x as f64 / self.resolution.y as f64;
        let viewport_width = self.viewport_height * aspect;

        let viewport_u = Direction::new(viewport_width, 0.0, 0.0);
        let viewport_v = Direction::new(0.0, -self.viewport_height, 0.0);

        let delta_u = viewport_u / self.resolution.x as f64;
        let delta_v = viewport_v / self.resolution.y as f64;

        let top_left = self.position.coords
            - Direction::new(0.0, 0.0, -self.focal_length)
            - viewport_u / 2.0
            - viewport_v / 2.0
            + (delta_u + delta_v) / 2.0;
        // Till here

        let pixel_center: Direction<f64> = top_left
            + delta_u * pixel.x as f64
            + delta_v * pixel.y as f64;

        let ray_direction = pixel_center - self.position.coords;

        return Ray {
            origin: self.position,
            direction: ray_direction,
        };
    }
}

