use bytemuck::{Pod, Zeroable};

use crate::types::{Position, Direction, PixelCoord};
use crate::compute::ComputeManager;
use crate::ray::Ray;

pub struct Camera {
    pub resolution: PixelCoord<u32>,
    pub focal_length: f32,
    pub viewport_height: f32,
    pub position: Position<f32>,
    pub rotation: Direction<f32>,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct GPUCameraParams {
    resolution: [u32; 2],
    position: [f32; 3],
    _pad1: f32,
    focal_length: f32,
    viewport_height: f32,
    _pad2: [f32; 2],
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            resolution: PixelCoord::<u32>::origin(),
            focal_length: 2.0, 
            viewport_height: 2.0,
            position: Position::<f32>::origin(),
            rotation: Direction::<f32>::zeros()
        }
    }

    pub fn gen_rays(&self, manager: &ComputeManager) {
        let params = GPUCameraParams::new(&self);
    }
}

impl GPUCameraParams {
    fn new(camera: &Camera) -> GPUCameraParams {
        GPUCameraParams {
            resolution: [camera.resolution.x, camera.resolution.y],
            position: camera.position.coords.into(),
            _pad1: 0.0,
            focal_length: camera.focal_length,
            viewport_height: camera.viewport_height,
            _pad2: [0.0, 0.0]
        }
    }
}

