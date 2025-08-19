use wgpu::{
    Buffer,
    ShaderStages,
    BindingType,
    BufferBindingType,
    BindGroupLayoutEntry,
};
use wgpu::util::DeviceExt;
use bytemuck::{
    Pod,
    cast_slice,
};
use std::collections::HashMap;

use crate::compute::{
    ComputeShader,
    ComputeManager
};

pub struct ComputeProgram {
    label: String,
    source_str: String,
    inputs: HashMap<u32, String>,
    outputs: HashMap<u32, String>,
    buffers: HashMap<u32, Buffer>,
    readback: HashMap<u32, Buffer>,
    shader: Option<ComputeShader>,
}

impl ComputeProgram {
    pub fn new(
        label: impl Into<String>, 
        source_str: impl Into<String>
    ) -> ComputeProgram {
        ComputeProgram {
            label: label.into(),
            source_str: source_str.into(),
            inputs: HashMap::new(),
            outputs: HashMap::new(),
            buffers: HashMap::new(),
            readback: HashMap::new(),
            shader: None
        }
    }

    pub fn with_input(&mut self, binding: u32, label: String) -> &mut Self {
        if self.inputs.contains_key(&binding) {
            panic!(
                "Input binding {binding} already exists in ComputeProgram {}.",
                self.label
            );
        }

        self.inputs.insert(binding, label);
        self
    }

    pub fn with_output(&mut self, binding: u32, label: String) -> &mut Self {
        if self.outputs.contains_key(&binding) {
            panic!(
                "Ouput binding {binding} already exists in ComputeProgram {}.",
                self.label
            );
        }

        self.outputs.insert(binding, label);
        self
    }

    pub fn compile(&mut self, manager: &ComputeManager) -> Result<(), String> {
        let mut layout_entries = vec![];

        for (binding, _label) in &self.inputs {
            layout_entries.push(BindGroupLayoutEntry {
                binding: *binding,
                visibility: ShaderStages::COMPUTE,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Storage { read_only: true },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            });
        }

        for (binding, _label) in &self.outputs {
            layout_entries.push(BindGroupLayoutEntry {
                binding: *binding,
                visibility: ShaderStages::COMPUTE,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Storage { read_only: false },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            });
        }

        let bind_group_layout = manager.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some(&self.label.to_string()),
            entries: &layout_entries,
        });

        let shader = ComputeShader::new(
            &self.label.to_string(),
            &self.source_str.to_string(),
            &bind_group_layout,
            &manager,
        ).expect("Could not compile compute shader!");
        self.shader = Some(shader);

        Ok(())
    }

    pub fn input_buffer<T: Pod>(&mut self, manager: &ComputeManager, binding: u32, contents: &[T]) {
        if !self.inputs.contains_key(&binding) {
            panic!("No input declared at binding {}.", binding);
        }

        let device = &manager.device;
        let contents_bytes = cast_slice(contents);
        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            // TODO change label formats.
            label: Some(&format!("InputBuffer(binding={binding})")),
            contents: contents_bytes,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
        });

        self.buffers.insert(binding, buffer);
    }

    pub fn init_output_buffers(
        &mut self,
        manager: &ComputeManager,
        sizes: HashMap<u32, u64>,
    ) {
        let device = &manager.device;

        for (&binding, _label) in &self.outputs {
            let size = sizes.get(&binding).copied().unwrap_or(1024) as u64;

            if self.buffers.contains_key(&binding) {
                continue;
            }

            let output_buffer = device.create_buffer(&wgpu::BufferDescriptor {
                label: Some(&format!("OutputBuffer(binding={binding})")),
                size,
                usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
                mapped_at_creation: false,
            });

            let readback_buffer = device.create_buffer(&wgpu::BufferDescriptor {
                label: Some(&format!("ReadbackBuffer(binding={binding})")),
                size,
                usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });

            self.buffers.insert(binding, output_buffer);
            self.readback.insert(binding, readback_buffer);
        }
    }

    pub fn dispatch<T: Pod>(
        &mut self,
        manager: &mut ComputeManager,
        dims: (u32, u32, u32),
    ) -> HashMap<u32, Vec<T>> {
        let shader = self.shader.as_ref().expect("Shader not compiled!");

        let mut entries = vec![];
        for (&binding, buffer) in &self.buffers {
            entries.push(wgpu::BindGroupEntry {
                binding,
                resource: buffer.as_entire_binding(),
            });
        }
        let layout = shader.pipeline.get_bind_group_layout(0);
        let bind_group = manager.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &layout,
            entries: &entries,
            label: Some(&format!("BindGroup({})", self.label)),
        });

        pollster::block_on(shader.dispatch::<T>(&bind_group, manager, dims));

        let mut results = HashMap::new();
        for (&binding, result_buffer) in &self.buffers {
            if !self.outputs.contains_key(&binding) {
                continue;
            }

            let result_readback = self.readback.get(&binding)
                .expect(&format!("No readback buffer for binding {}", binding));

            let size = result_buffer.size();

            let mut encoder = manager.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("CopyEncoder"),
            });
            encoder.copy_buffer_to_buffer(result_buffer, 0, result_readback, 0, size);
            manager.queue.submit(Some(encoder.finish()));

            let slice = result_readback.slice(..);
            slice.map_async(wgpu::MapMode::Read, |_| {});
            let _ = manager.device.poll(wgpu::MaintainBase::Wait);

            let data = slice.get_mapped_range();
            let typed: Vec<T> = bytemuck::cast_slice(&data).to_vec();
            drop(data);
            result_readback.unmap();

            results.insert(binding, typed);
        }

        results
    }
}

