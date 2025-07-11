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
    BindGroupLayout,
    CommandEncoder,
    CommandEncoderDescriptor,
};

use crate::compute::ComputeShader;

pub struct ComputeManager {
    pub instance: Instance,
    pub adapter: Adapter,
    pub device: Device,
    pub queue: Queue,
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

    pub fn load_shader(
        &mut self,
        label: &str,
        source: &str,
        bind_group_layout: &BindGroupLayout,
    ) -> Result<ComputeShader, std::io::Error> {
        ComputeShader::new(
            label, 
            source,
            &bind_group_layout, 
            &self
        )
    }

    pub fn request_encoder(&mut self) -> CommandEncoder {
        let encoder = self.device.create_command_encoder(&CommandEncoderDescriptor {
            label: Some("Nothing"),
        });
        encoder
    }
}

