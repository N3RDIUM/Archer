use wgpu::*;
use wgpu::util::DeviceExt;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct Ray {
    origin: [f32; 3],
    _pad1: f32,
    dir: [f32; 3],
    _pad2: f32,
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct Sphere {
    center: [f32; 3],
    radius: f32,
}

pub struct ComputeManager {}

pub async fn run() -> f32 {
    // ==== WGPU Init ====
    let instance = Instance::new(&wgpu::InstanceDescriptor {
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

    // ==== Data ====
    let ray = Ray {
        origin: [0.0, 0.0, -5.0],
        _pad1: 0.0,
        dir: [0.0, 0.0, 1.0],
        _pad2: 0.0,
    };

    let sphere = Sphere {
        center: [0.0, 0.0, 0.0],
        radius: 1.0,
    };

    let result = -1.0f32;

    // ==== Buffers ====
    let ray_buffer = device.create_buffer_init(&util::BufferInitDescriptor {
        label: Some("Ray Buffer"),
        contents: bytemuck::cast_slice(&[ray]),
        usage: BufferUsages::STORAGE,
    });

    let sphere_buffer = device.create_buffer_init(&util::BufferInitDescriptor {
        label: Some("Sphere Buffer"),
        contents: bytemuck::cast_slice(&[sphere]),
        usage: BufferUsages::STORAGE,
    });

    let result_buffer = device.create_buffer_init(&util::BufferInitDescriptor {
        label: Some("Result Buffer"),
        contents: bytemuck::cast_slice(&[result]),
        usage: BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
    });

    let result_readback = device.create_buffer(&BufferDescriptor {
        label: Some("Result Readback Buffer"),
        size: std::mem::size_of::<f32>() as u64,
        usage: BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
        mapped_at_creation: false,
    });

    // ==== Shader ====
    let shader = device.create_shader_module(ShaderModuleDescriptor {
        label: Some("Ray-Sphere Intersection Compute"),
        source: ShaderSource::Wgsl(include_str!("./shaders/ray-sphere.wgsl").into()),
    });

    let bind_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
        label: Some("Bind Group Layout"),
        entries: &[
            BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::COMPUTE,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Storage { read_only: true },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
            BindGroupLayoutEntry {
                binding: 1,
                visibility: ShaderStages::COMPUTE,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Storage { read_only: true },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
            BindGroupLayoutEntry {
                binding: 2,
                visibility: ShaderStages::COMPUTE,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Storage { read_only: false },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
        ],
    });

    let bind_group = device.create_bind_group(&BindGroupDescriptor {
        label: Some("Bind Group"),
        layout: &bind_group_layout,
        entries: &[
            BindGroupEntry {
                binding: 0,
                resource: ray_buffer.as_entire_binding(),
            },
            BindGroupEntry {
                binding: 1,
                resource: sphere_buffer.as_entire_binding(),
            },
            BindGroupEntry {
                binding: 2,
                resource: result_buffer.as_entire_binding(),
            },
        ],
    });

    let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
        label: Some("Pipeline Layout"),
        bind_group_layouts: &[&bind_group_layout],
        push_constant_ranges: &[],
    });

    let pipeline = device.create_compute_pipeline(&ComputePipelineDescriptor {
        label: Some("Compute Pipeline"),
        layout: Some(&pipeline_layout),
        module: &shader,
        entry_point: Some("main"),
        compilation_options: PipelineCompilationOptions::default(),
        cache: None,
    });

    // ==== Dispatch ====
    let mut encoder = device.create_command_encoder(&CommandEncoderDescriptor {
        label: Some("Encoder"),
    });

    {
    let mut cpass = encoder.begin_compute_pass(&ComputePassDescriptor {
        label: Some("Compute Pass"),
        timestamp_writes: None,
    });
    cpass.set_pipeline(&pipeline);
        cpass.set_bind_group(0, &bind_group, &[]);
        cpass.dispatch_workgroups(1, 1, 1);
    }

    encoder.copy_buffer_to_buffer(&result_buffer, 0, &result_readback, 0, 4);
    queue.submit(Some(encoder.finish()));

    // ==== Read Result ====
    let buffer_slice = result_readback.slice(..);
    buffer_slice.map_async(MapMode::Read, |_| {});
    let _ = device.poll(MaintainBase::Wait);

    let data = buffer_slice.get_mapped_range();
    let result = bytemuck::cast_slice::<u8, f32>(&data)[0];

    drop(data);
    result_readback.unmap();

    result
}

