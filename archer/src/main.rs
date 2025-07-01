use archer::camera::Camera;
use archer::compute::ComputeManager;
use archer::types::PixelCoord;

fn main() {
    let mut manager = pollster::block_on(ComputeManager::new());

    let mut camera = Camera::new(&mut manager);
    camera.resolution = PixelCoord::new(1920, 1080);
    loop {
        camera.gen_rays();
    }
}

