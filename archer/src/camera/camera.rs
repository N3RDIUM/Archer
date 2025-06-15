use crate::types::{Position, Direction, PixelCoord};
use crate::ray::Ray;

pub struct Camera {
    pub resolution: PixelCoord<u16>,
    pub focal_length: f32,
    pub viewport_height: f32,
    pub position: Position<f32>,
    pub rotation: Direction<f32>,
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            resolution: PixelCoord::<u16>::origin(),
            focal_length: 2.0, 
            viewport_height: 2.0,
            position: Position::<f32>::origin(),
            rotation: Direction::<f32>::zeros()
        }
    }

    pub fn get_ray_at(&self, pixel: PixelCoord<u16>) -> Ray {
        // TODO: this should only be computed once unless parameters change
        let aspect: f32 = self.resolution.x as f32 / self.resolution.y as f32;
        let viewport_width = self.viewport_height * aspect;

        let viewport_u = Direction::new(viewport_width, 0.0, 0.0);
        let viewport_v = Direction::new(0.0, -self.viewport_height, 0.0);

        let delta_u = viewport_u / self.resolution.x as f32;
        let delta_v = viewport_v / self.resolution.y as f32;

        let top_left = self.position.coords
            - Direction::new(0.0, 0.0, -self.focal_length)
            - viewport_u / 2.0
            - viewport_v / 2.0
            + (delta_u + delta_v) / 2.0;
        // Till here

        let pixel_center: Direction<f32> = top_left
            + delta_u * pixel.x as f32
            + delta_v * pixel.y as f32;

        let ray_direction = pixel_center - self.position.coords;

        return Ray {
            origin: self.position,
            direction: ray_direction,
        };
    }
}

