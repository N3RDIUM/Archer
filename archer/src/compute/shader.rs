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
    PollType,
};

use crate::compute::ComputeManager;

pub struct ComputeShader {
    label: String,
    pipeline: ComputePipeline,
}

impl ComputeShader {
    pub fn new(
        label: &str,
        source: &str,
        bind_group_layout: BindGroupLayout,
        manager: &ComputeManager
    ) -> Result<ComputeShader, std::io::Error> {
        let source = ShaderSource::Wgsl(source.into());

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

    pub async fn dispatch(
        &self,
        bind_group: &BindGroup,
        manager: &mut ComputeManager,
        pass: &mut ComputePass<'_>,
        result_buffer: &Buffer,
        result_readback: &Buffer,
        result_size: u64
    ) {
        pass.set_pipeline(&self.pipeline);
        pass.set_bind_group(0, bind_group, &[]);
        pass.dispatch_workgroups(1, 1, 1);

        let mut encoder = manager.device.create_command_encoder(&CommandEncoderDescriptor {
            label: Some(self.label.as_str()),
        });

        encoder.copy_buffer_to_buffer(
            &result_buffer, 0,
            &result_readback, 0,
            result_size
        );
        let index = manager.queue.submit(Some(encoder.finish()));

        manager.device.poll(PollType::WaitForSubmissionIndex(index));
    }
}

