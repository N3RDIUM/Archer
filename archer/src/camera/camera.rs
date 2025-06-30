use std::collections::HashMap;

use bytemuck::{Pod, Zeroable, cast_slice};
use pollster::block_on;
use wgpu::*;
use wgpu::util::*;

use crate::types::{Position, Direction, PixelCoord};
use crate::compute::{ComputeManager, ComputeProgram};
use crate::ray::Ray;

pub struct Camera<'a> {
    pub resolution: PixelCoord<u32>,
    pub focal_length: f32,
    pub viewport_height: f32,
    pub position: Position<f32>,
    pub rotation: Direction<f32>,
    program: ComputeProgram,
    manager: &'a ComputeManager,
}

#[repr(C, align(16))]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct GPUCameraParams {
    resolution: [u32; 2],
    _pad0: [u32; 2],
    position: [f32; 3],
    _pad1: f32,
    focal_length: f32,
    viewport_height: f32,
    _pad2: [f32; 2],
}

impl Camera<'_> {
    pub fn new(manager: &mut ComputeManager) -> Camera {
        let mut program = ComputeProgram::new(
            "Camera",
            include_str!("./raygen.wgsl")
        );

        program
            .with_input(0, String::from("Camera Parameters"))
            .with_output(1, String::from("Rays"))
            .compile(&manager)
            .expect("Could not compile ray generator program!");

        Camera {
            resolution: PixelCoord::<u32>::origin(),
            focal_length: 2.0, 
            viewport_height: 2.0,
            position: Position::<f32>::origin(),
            rotation: Direction::<f32>::zeros(),
            program,
            manager
        }
    }

    pub fn gen_rays(&mut self) {
        let parameters = GPUCameraParams::new(&self);
        self.program.input_buffer(&self.manager, 0, [parameters]);

        let resolution = &self.resolution;
        let total_pixels = (resolution.x * resolution.y) as usize;
        let output_size = (std::mem::size_of::<Ray>() * total_pixels) as u64;
        let size_map = HashMap::from([ (1, output_size) ]);
        self.program.init_output_buffers(&self.manager, size_map);
    }
}

impl GPUCameraParams {
    fn new(camera: &Camera) -> GPUCameraParams {
        GPUCameraParams {
            resolution: [camera.resolution.x, camera.resolution.y],
            _pad0: [0, 0],
            position: camera.position.coords.into(),
            _pad1: 0.0,
            focal_length: camera.focal_length,
            viewport_height: camera.viewport_height,
            _pad2: [0.0, 0.0]
        }
    }
}

