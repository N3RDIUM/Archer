use crate::types::{Position, Direction, PixelCoord};
use crate::compute::manager::ComputeManager;
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

    pub fn gen_rays(&self, manager: &ComputeManager) {

    }
}

