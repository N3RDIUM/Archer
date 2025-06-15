use std::time::Instant;

use archer::camera::Camera;
use archer::compute::manager::ComputeManager;
use archer::types::PixelCoord;

fn main() {
    let manager = pollster::block_on(ComputeManager::new());

    let now = Instant::now();

    let mut camera = Camera::new();
    camera.resolution = PixelCoord::new(1920, 1080);
    let rays = camera.gen_rays(&manager);

    let elapsed = now.elapsed().as_secs_f64();
    let fps = 1.0 / elapsed;
    print!("{elapsed} sec, {fps} fps");
}

