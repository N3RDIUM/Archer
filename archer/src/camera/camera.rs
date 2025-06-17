use std::time::Instant;
use bytemuck::{Pod, Zeroable, cast_slice};
use pollster::block_on;
use wgpu::*;
use wgpu::util::*;

use crate::types::{Position, Direction, PixelCoord};
use crate::compute::ComputeManager;
use crate::ray::{Ray, GPURay};

pub struct Camera {
    pub resolution: PixelCoord<u32>,
    pub focal_length: f32,
    pub viewport_height: f32,
    pub position: Position<f32>,
    pub rotation: Direction<f32>,
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

    pub fn gen_rays(&self, mut manager: &mut ComputeManager) {
        let bind_group_layout = manager.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Camera Raygen BindGroupLayout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: false },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        });

        let shader = manager.load_shader(
            "Camera Raygen",
            include_str!("./ray_generator.wgsl"),
            &bind_group_layout
        ).expect("Failed to load shader!");

        let params = GPUCameraParams::new(self);

        let params_buffer = manager.device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Camera Parameter Buffer"),
            contents: cast_slice(&[params]),
            usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
        });

        let total_pixels = (self.resolution.x * self.resolution.y) as usize;
        let result_data = vec![GPURay::zeroed(); total_pixels];

        let result_buffer = manager.device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Camera Raygen Result Buffer"),
            contents: cast_slice(&result_data),
            usage: BufferUsages::STORAGE | BufferUsages::COPY_SRC,
        });

        let result_readback = manager.device.create_buffer(&BufferDescriptor {
            label: Some("Camera Raygen Result Readback Buffer"),
            size: (std::mem::size_of::<GPURay>() * total_pixels) as u64,
            usage: BufferUsages::COPY_DST | BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });

        let bind_group = manager.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Camera Raygen BindGroup"),
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: params_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: result_buffer.as_entire_binding(),
                },
            ],
        });

        let now = Instant::now();
        let result = block_on(shader.dispatch::<GPURay>(
            &bind_group,
            &mut manager,
            &result_buffer,
            &result_readback,
            (std::mem::size_of::<GPURay>() * total_pixels) as u64,
            (
                (self.resolution.x + 7) / 16,
                (self.resolution.y + 7) / 16,
                1
            ),
        ));
        let elapsed = now.elapsed().as_secs_f64();
        let fps = 1.0 / elapsed;
        println!("{elapsed} sec, {fps} fps");
        println!("{:?}", result.len());
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

