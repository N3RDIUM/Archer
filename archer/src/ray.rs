use crate::types::{Position, Direction};
use crate::compute::to_gpu::ToGPU;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Position<f32>,
    pub direction: Direction<f32>
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct GPURay {
    pub origin: [f32; 3],
    _pad1: f32,
    pub dir: [f32; 3],
    _pad2: f32,
}

impl Ray {
    pub fn new() -> Ray {
        return Ray {
            origin: Position::<f32>::origin(),
            direction: Direction::<f32>::zeros(),
        }
    }

    pub fn at(self, t: f32) -> Position<f32> {
        return self.origin + self.direction * t;
    }
}

impl ToGPU for Ray {
    type GPUType = GPURay;

    fn to_gpu(&self) -> GPURay {
        GPURay::new(self.origin, self.direction)
    }
}

impl GPURay {
    pub fn new(origin: Position<f32>, direction: Direction<f32>) -> GPURay {
        GPURay {
            origin: origin.coords.into(),
            _pad1: 0.0,
            dir: direction.into(),
            _pad2: 0.0
        }
    }
}

