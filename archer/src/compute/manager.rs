use wgpu::{
    Instance,
    Adapter,
    Device,
    Queue,
    InstanceDescriptor,
    Backends,
    InstanceFlags,
    RequestAdapterOptions,
    DeviceDescriptor,
    CommandEncoder,
    CommandEncoderDescriptor,
    CommandBuffer,
    SubmissionIndex,
    BindGroupLayout,
};
use std::collections::HashMap;

use crate::compute::ComputeShader;

pub struct ComputeManager<'a> {
    pub instance: Instance,
    pub adapter: Adapter,
    pub device: Device,
    pub queue: Queue,
    shaders: HashMap<&'a str, ComputeShader>,
}

pub trait ToGPU {
    type GPUType: bytemuck::Pod;
    fn to_gpu(&self) -> Self::GPUType;
}

pub trait ToCPU {
    type CPUType;
    fn to_gpu(&self) -> Self::CPUType;
}

impl ComputeManager<'_> {
    pub async fn new() -> ComputeManager<'static> {
        let instance = Instance::new(&InstanceDescriptor {
            backends: Backends::VULKAN,
            flags: InstanceFlags::empty(),
            backend_options: Default::default(),
        });

        let adapter = instance
            .request_adapter(&RequestAdapterOptions::default())
            .await
            .expect("No adapter found");

        let (device, queue) = adapter
            .request_device(&DeviceDescriptor::default())
            .await
            .expect("Device request failed");

        ComputeManager {
            instance,
            adapter,
            device,
            queue,
            shaders: HashMap::new(),
        }
    }

    pub fn load_shader(
        &mut self,
        label: &str,
        source: &str,
        bind_group_layout: &BindGroupLayout,
    ) {
        let shader = ComputeShader::new(label, source, &bind_group_layout, &self);
    }
}

