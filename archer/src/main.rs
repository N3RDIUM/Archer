use std::time::Instant;
use image::{ImageBuffer, RgbImage};

use archer::camera::Camera;
use archer::vectors;
use vectors::Vec2;
use vectors::Vec3;
use vectors::Ray;

fn do_it(camera: &Camera, x: i32, y: i32) -> Ray {
    return camera.get_ray(Vec2 { x: x as f32, y: y as f32 })
}

fn main() {
    const RESOLUTION: Vec2 = Vec2 { x: 1920.0, y: 1080.0 };
    let mut image: RgbImage = ImageBuffer::new(RESOLUTION.x as u32, RESOLUTION.y as u32);

    // TODO: Create a nice ::new() impl method for Camera...
    // TODO: ...so that we dont have to deal with this mess here.
    let mut cam = Camera {
       resolution: RESOLUTION,
       focal_length: 1.0,
       viewport_height: 2.0,
       position: Vec3::fill(0.0),
       rotation: Vec3::fill(0.0),

       aspect: 0.0,
       viewport_width: 1.0,
       viewport_u: Vec3::fill(0.0),
       viewport_v: Vec3::fill(0.0),
       pixel_delta_u: Vec3::fill(0.0),
       pixel_delta_v: Vec3::fill(0.0),
       top_left: Vec3::fill(0.0),
       top_left_location: Vec3::fill(0.0)
    };
    cam.update();
    println!("Current camera state: {cam:?}");
    
    let now = Instant::now();

    for x in 0..RESOLUTION.x as i32 {
        for y in 0..RESOLUTION.y as i32{
            let _r: Ray = do_it(&cam, x, y);      
            *image.get_pixel_mut(x as u32, y as u32) = image::Rgb([255; 3]);
        }
    }
    
    image.save("output.png").unwrap();

    let elapsed = now.elapsed().as_secs_f64();
    let fps: f64 = 1.0 / elapsed;
    println!("One frame took {elapsed} seconds. That's {fps} FPS!");
}
