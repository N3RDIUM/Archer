use wgpu::{
    BindGroupLayout,
    ComputePipeline,
    ShaderSource,
    ShaderModuleDescriptor,
    PipelineLayoutDescriptor,
    ComputePipelineDescriptor,
    PipelineCompilationOptions,
    BindGroup,
    Buffer,
    CommandEncoderDescriptor,
    PollType,
};

use crate::compute::ComputeManager;

pub struct ComputeShader {
    pub label: String,
    pub pipeline: ComputePipeline,
}

impl ComputeShader {
    pub fn new(
        label: &str,
        source_str: &str,
        bind_group_layout: &BindGroupLayout,
        manager: &ComputeManager
    ) -> Result<ComputeShader, std::io::Error> {
        let source = ShaderSource::Wgsl(source_str.into());

        let shader = manager.device.create_shader_module(ShaderModuleDescriptor { 
            label: Some(label),
            source,
        });

        let pipeline_layout = manager.device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some(label),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let pipeline = manager.device.create_compute_pipeline(&ComputePipelineDescriptor {
            label: Some(label),
            layout: Some(&pipeline_layout),
            module: &shader,
            entry_point: Some("main"),
            compilation_options: PipelineCompilationOptions::default(),
            cache: None,
        });

        Ok(ComputeShader {
            label: label.to_string(),
            pipeline
        })
    }

    pub async fn dispatch<T: bytemuck::Pod>(
        &self,
        bind_group: &BindGroup,
        manager: &mut ComputeManager,
        dispatch_dims: (u32, u32, u32),
    ) {
        let mut encoder = manager.device.create_command_encoder(&CommandEncoderDescriptor {
            label: Some(self.label.as_str()),
        });

        // TODO: Let ComputeManager handle this
        {
            let mut cpass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("Compute Pass"),
                timestamp_writes: None,
            });
            cpass.set_pipeline(&self.pipeline);
            cpass.set_bind_group(0, bind_group, &[]);
            cpass.dispatch_workgroups(dispatch_dims.0, dispatch_dims.1, dispatch_dims.2);
        }

        let index = manager.queue.submit(Some(encoder.finish()));
        let _ = manager.device.poll(PollType::WaitForSubmissionIndex(index));
    }
}

