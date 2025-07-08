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
    ) -> Vec<T> {
        // Ensure the shader has been compiled
        let shader = self.shader.as_ref().expect("Shader not compiled!");

        // Collect entries for bind group
        let mut entries = vec![];

        for (&binding, buffer) in &self.buffers {
            entries.push(wgpu::BindGroupEntry {
                binding,
                resource: buffer.as_entire_binding(),
            });
        }

        // Create bind group
        let layout = shader.pipeline.get_bind_group_layout(0);
        let bind_group = manager.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &layout,
            entries: &entries,
            label: Some(&format!("BindGroup({})", self.label)),
        });

        // Get result buffer and readback buffer
        let (&result_binding, result_buffer) = self
            .outputs
            .keys()
            .next()
            .and_then(|k| self.buffers.get_key_value(k))
            .expect("No output buffers to dispatch into!");

        let result_readback = self.readback.get(&result_binding)
            .expect("No readback buffer for output binding!");

        let size = result_buffer.size();

        // Dispatch using shader
        pollster::block_on(shader.dispatch::<T>(
            &bind_group,
            manager,
            result_buffer,
            result_readback,
            size,
            dims,
        ))
    }
}

