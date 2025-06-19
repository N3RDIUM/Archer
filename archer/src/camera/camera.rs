use bytemuck::{Pod, Zeroable, cast_slice};
use pollster::block_on;
use wgpu::*;
use wgpu::util::*;

use crate::types::{Position, Direction, PixelCoord};
use crate::compute::{ComputeManager, ComputeShader};
use crate::ray::GPURay;

pub struct Camera<'a> {
    pub resolution: PixelCoord<u32>,
    pub focal_length: f32,
    pub viewport_height: f32,
    pub position: Position<f32>,
    pub rotation: Direction<f32>,

    raygen: CameraRaygen<'a>,
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

pub struct CameraRaygen<'a> {
    bind_group_layout: BindGroupLayout,
    shader: ComputeShader,
    manager: &'a mut ComputeManager,

    params_buffer: Option<Buffer>,
    result_buffer: Option<Buffer>,
    result_readback: Option<Buffer>,
    bind_group: Option<BindGroup>,
}

impl Camera<'_> {
    pub fn new(manager: &mut ComputeManager) -> Camera {
        let raygen = CameraRaygen::new(manager);
        
        Camera {
            resolution: PixelCoord::<u32>::origin(),
            focal_length: 2.0, 
            viewport_height: 2.0,
            position: Position::<f32>::origin(),
            rotation: Direction::<f32>::zeros(),
            raygen
        }
    }

    pub fn init(&mut self) {
        let params = GPUCameraParams::new(self);
        self.raygen.init_buffers(&self.resolution, params);
    }

    pub fn gen_rays(&mut self) {
        let params = GPUCameraParams::new(self);
        self.raygen.gen_rays(&self.resolution, params);
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

impl CameraRaygen<'_> {
    pub fn new(manager: &mut ComputeManager) -> CameraRaygen {
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

        CameraRaygen {
            bind_group_layout,
            shader,
            params_buffer: None,
            result_buffer: None,
            result_readback: None,
            bind_group: None,
            manager
        }
    }
    
    fn init_buffers(&mut self, resolution: &PixelCoord<u32>, params: GPUCameraParams) {
        self.params_buffer = Some(self.manager.device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Camera Parameter Buffer"),
            contents: cast_slice(&[params]),
            usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
        }));

        let total_pixels = (resolution.x * resolution.y) as usize;
        let result_data = vec![GPURay::zeroed(); total_pixels];

        self.result_buffer = Some(self.manager.device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Camera Raygen Result Buffer"),
            contents: cast_slice(&result_data),
            usage: BufferUsages::STORAGE | BufferUsages::COPY_SRC,
        }));

        self.result_readback = Some(self.manager.device.create_buffer(&BufferDescriptor {
            label: Some("Camera Raygen Result Readback Buffer"),
            size: (std::mem::size_of::<GPURay>() * total_pixels) as u64,
            usage: BufferUsages::COPY_DST | BufferUsages::MAP_READ,
            mapped_at_creation: false,
        }));

        self.bind_group = Some(self.manager.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Camera Raygen BindGroup"),
            layout: &self.bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: self
                        .params_buffer
                        .as_ref()
                        .expect("Raygen Param Buffer Not Created")
                        .as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: self
                        .result_buffer
                        .as_ref()
                        .expect("Raygen Result Buffer Not Created")
                        .as_entire_binding(),
                },
            ],
        }));
    }

    pub fn gen_rays(&mut self, resolution: &PixelCoord<u32>, params: GPUCameraParams) {
        let total_pixels = (resolution.x * resolution.y) as usize;

        self.manager.queue.write_buffer(
            self.params_buffer
                .as_ref()
                .expect("Raygen Param Buffer Not Created"),
            0,
            cast_slice(&[params]),
        );

        let queue_index = self.shader.dispatch::<GPURay>(
            self
                .bind_group
                .as_ref()
                .expect("Raygen Buffers Not Initialized"),
            &mut self.manager,
            &self
                .result_buffer
                .as_ref()
                .expect("Raygen Buffers Not Initialized"),
            &self
                .result_readback
                .as_ref()
                .expect("Raygen Buffers Not Initialized"),
            (std::mem::size_of::<GPURay>() * total_pixels) as u64,
            (
                (resolution.x + 7) / 16,
                (resolution.y + 7) / 16,
                1
            ),
        );

        let result = block_on(queue_index);
        println!("{:?}", result.len());
    }
}

