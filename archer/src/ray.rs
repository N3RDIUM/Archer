use crate::types::{Position, Direction};
use crate::compute::{ToGPU, ToCPU};

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
    pub direction: [f32; 3],
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
        GPURay::new(&self)
    }
}

impl GPURay {
    pub fn new(ray: &Ray) -> GPURay {
        GPURay {
            origin: ray.origin.coords.into(),
            _pad1: 0.0,
            direction: ray.direction.into(),
            _pad2: 0.0
        }
    }
}

impl ToCPU for GPURay {
    type CPUType = Ray;

    fn to_gpu(&self) -> Self::CPUType {
        Ray {
            origin: Position::from(self.origin),
            direction: Direction::from(self.direction)
        }
    }
}

