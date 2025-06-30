use wgpu::{
    Buffer,
    ShaderStages,
    BindingType,
    BufferBindingType,
    BindGroupLayoutEntry,
};
use bytemuck::{
    Pod,
    bytes_of,
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

    pub fn with_input(&mut self, label: String, binding: u32) -> Result<(), String> {
        if self.inputs.contains_key(&binding) {
            return Err(format!(
                "Input binding {binding} already exists in ComputeProgram {}.",
                self.label
            ));
        }

        self.inputs.insert(binding, label);
        Ok(())
    }

    pub fn with_output(&mut self, label: String, binding: u32) -> Result<(), String> {
        if self.outputs.contains_key(&binding) {
            return Err(format!(
                "Ouput binding {binding} already exists in ComputeProgram {}.",
                self.label
            ));
        }

        self.outputs.insert(binding, label);
        Ok(())
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

    fn input_buffer<T: Pod>(&mut self, manager: &ComputeManager, binding: u32, contents: T) -> Result<(), String> {
        if !self.inputs.contains_key(&binding) {
            return Err(format!("No input declared at binding {}.", binding));
        }

        let device = &manager.device;
        let contents_bytes = bytes_of(&contents);
        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            // TODO change label formats.
            label: Some(&format!("InputBuffer(binding={binding})")),
            contents: contents_bytes,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
        });

        self.buffers.insert(binding, buffer);
        Ok(())
    }

    fn init_output_buffers(
        &mut self,
        manager: &ComputeManager,
        sizes: HashMap<u32, u64>,
    ) -> Result<(), String> {
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

        Ok(())
    }
}

