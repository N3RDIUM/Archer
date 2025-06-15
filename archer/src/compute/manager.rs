use wgpu::{
    Instance,
    Adapter,
    Device,
    Queue,
    InstanceDescriptor,
    Backends,
    InstanceFlags,
    RequestAdapterOptions,
    DeviceDescriptor
};

pub struct ComputeManager {
    instance: Instance,
    adapter: Adapter,
    device: Device,
    queue: Queue
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
            queue
        }
    }
}

