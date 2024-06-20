use std::time::Instant;
use image::{ImageBuffer, RgbImage};

use archer::camera::Camera;
use archer::vectors;
use vectors::Vec2;
use vectors::Ray;

fn do_it(camera: &Camera, x: u32, y: u32) -> Ray {
    return camera.get_ray(Vec2 { x: x as f32, y: y as f32 })
}

fn main() {
    const RESOLUTION: [u32; 2] = [1920, 1080];
    let mut image: RgbImage = ImageBuffer::new(RESOLUTION[0], RESOLUTION[1]);

    let mut cam = Camera::new(RESOLUTION);
    cam.update();
    println!("Current camera state: {cam:?}");
    
    let now = Instant::now();

    for x in 0..RESOLUTION[0] {
        for y in 0..RESOLUTION[1] {
            let _r: Ray = do_it(&cam, x, y);      
            *image.get_pixel_mut(x, y) = image::Rgb([255; 3]);
        }
    }
    
    image.save("output.png").unwrap();

    let elapsed = now.elapsed().as_secs_f64();
    let fps: f64 = 1.0 / elapsed;
    println!("One frame took {elapsed} seconds. That's {fps} FPS!");
}
