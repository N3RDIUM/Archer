pub mod manager;
pub use manager::{
    ComputeManager,
    ToGPU,
    ToCPU
};

pub mod shader;
pub use shader::ComputeShader;

