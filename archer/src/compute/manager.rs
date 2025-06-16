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
};
use std::option::Option;

pub struct ComputeManager {
    pub instance: Instance,
    pub adapter: Adapter,
    pub device: Device,
    pub queue: Queue,
}

pub trait ToGPU {
    type GPUType: bytemuck::Pod;
    fn to_gpu(&self) -> Self::GPUType;
}

pub trait ToCPU {
    type CPUType;
    fn to_gpu(&self) -> Self::CPUType;
}

impl ComputeManager {
    pub async fn new() -> ComputeManager {
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
        }
    }
}

