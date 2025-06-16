use wgpu::{
    ShaderModule,
    BindGroupLayout,
    PipelineLayout,
    ComputePipeline,
    ShaderSource,
    ShaderModuleDescriptor,
    PipelineLayoutDescriptor,
    ComputePipelineDescriptor,
    PipelineCompilationOptions,
    BindGroup,
    ComputePass,
    Buffer,
    CommandEncoderDescriptor,
};
use wgpu::MaintainBase::WaitForSubmissionIndex;
use std::fs;

use crate::compute::ComputeManager;

pub struct ComputeShader {
    label: String,
    path: String,
    shader: ShaderModule,
    bind_group_layout: BindGroupLayout,
    pipeline_layout: PipelineLayout,
    pipeline: ComputePipeline,
}

impl ComputeShader {
    fn new(label: String, path: String, bind_group_layout: BindGroupLayout, manager: &ComputeManager) -> ComputeShader {
        let source_str = match fs::read_to_string(&path) {
            Ok(src) => src,
            Err(err) => {
                eprintln!("Failed to read shader {}: {}", path, err);
                panic!();
            },
        };
        let source = ShaderSource::Wgsl(source_str.into());

        let shader = manager.device.create_shader_module(ShaderModuleDescriptor { 
            label: Some(label.as_str()), 
            source,
        });

        let pipeline_layout = manager.device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some(format!("{label} Pipeline Layout").as_str()),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let pipeline = manager.device.create_compute_pipeline(&ComputePipelineDescriptor {
            label: Some(format!("{label} Pipeline").as_str()),
            layout: Some(&pipeline_layout),
            module: &shader,
            entry_point: Some("main"),
            compilation_options: PipelineCompilationOptions::default(),
            cache: None,
        });

        ComputeShader {
            label,
            path,
            shader,
            bind_group_layout,
            pipeline_layout,
            pipeline
        }
    }

    pub async fn dispatch(
        &self,
        bind_group: &BindGroup,
        manager: &mut ComputeManager,
        pass: &mut ComputePass<'_>,
        result_buffer: &Buffer,
        result_readback: &Buffer,
    ) {
        pass.set_pipeline(&self.pipeline);
        pass.set_bind_group(0, bind_group, &[]);
        pass.dispatch_workgroups(1, 1, 1);

        let mut encoder = manager.device.create_command_encoder(&CommandEncoderDescriptor {
            label: Some(format!("{:?} Encoder", self.label).as_str()),
        });

        encoder.copy_buffer_to_buffer(
            &result_buffer, 0,
            &result_readback, 0,
            result_buffer.size() as u64
        );
        let index = manager.queue.submit(Some(encoder.finish()));

        manager.device.poll(WaitForSubmissionIndex(index));
    }
}

