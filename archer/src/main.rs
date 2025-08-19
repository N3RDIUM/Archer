use archer::camera::Camera;
use archer::compute::ComputeManager;
use archer::types::PixelCoord;

use std::time::Instant;

fn main() {
    let mut manager = pollster::block_on(ComputeManager::new());

    let mut camera = Camera::new(&mut manager);
    camera.resolution = PixelCoord::new(1920, 1080);

    loop {
        let now = Instant::now();

        let rays = camera.gen_rays();

        let time = now.elapsed().as_secs_f64();
        let fps = 1.0 / time;
        println!("{fps}");
    }
}

