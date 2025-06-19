use archer::camera::Camera;
use archer::compute::ComputeManager;
use archer::types::PixelCoord;

fn main() {
    let mut manager = pollster::block_on(ComputeManager::new());

    let mut camera = Camera::new();
    camera.resolution = PixelCoord::new(256, 256);
    let _rays = camera.gen_rays(&mut manager);
}

