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
use std::time::Instant;

use crate::compute::ComputeManager;

pub struct ComputeShader {
    label: String,
    pipeline: ComputePipeline,
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
        result_buffer: &Buffer,
        result_readback: &Buffer,
        result_size: u64,
        dispatch_dims: (u32, u32, u32),
    ) -> Vec<T> {
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


        encoder.copy_buffer_to_buffer(
            &result_buffer, 0,
            &result_readback, 0,
            result_size
        );

        let now = Instant::now();
        let index = manager.queue.submit(Some(encoder.finish()));

        let buffer_slice = result_readback.slice(..);
        buffer_slice.map_async(wgpu::MapMode::Read, |_| {});
        let _ = manager.device.poll(PollType::WaitForSubmissionIndex(index));

        let elapsed = now.elapsed().as_secs_f64();
        let fps = 1.0 / elapsed;
        println!("{elapsed} sec, {fps} fps");

        let data = buffer_slice.get_mapped_range();
        let result: Vec<T> = bytemuck::cast_slice(&data).to_vec();
        drop(data);
        result_readback.unmap();

        result
    }
}

