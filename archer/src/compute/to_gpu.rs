pub trait ToGPU {
    type GPUType: bytemuck::Pod;
    fn to_gpu(&self) -> Self::GPUType;
}
