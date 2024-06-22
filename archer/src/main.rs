use std::time::Instant;
use image::{ImageBuffer, RgbImage};

use archer::models::sphere::Sphere;
use archer::models::base::Model;
use archer::camera::Camera;
use archer::ray::Ray;
use archer::vectors;
use vectors::Vec2;
use vectors::Vec3;

fn do_it(camera: &Camera, x: u32, y: u32) -> Ray {
    return camera.get_ray(Vec2 { x: x as f32, y: y as f32 })
}

fn main() {
    const RESOLUTION: [u32; 2] = [1920, 1080];
    let white: Vec3 = Vec3::fill(255.0);
    let blue: Vec3 = Vec3 { x: 0.5 * 255.0, y: 0.7 * 255.0, z: 255.0 };

    let mut image: RgbImage = ImageBuffer::new(RESOLUTION[0], RESOLUTION[1]);
    let mut cam = Camera::new(RESOLUTION);
    cam.update();

    let sphere: Sphere = Sphere {
        radius: 0.5,
        position: Vec3 { x: 0.0, y: 0.0, z: -1.0 }
    };

    let now = Instant::now();

    for x in 0..RESOLUTION[0] {
        for y in 0..RESOLUTION[1] {
            let ray: Ray = do_it(&cam, x, y);
            
            let norm = ray.direction.normalize();
            let a = 0.5 * (norm.y + 1.0);
            let color: Vec3 = Vec3::fill(1.0 - a) * white + Vec3::fill(a) * blue;
            let mut final_color: [u8; 3] = [color.x as u8, color.y as u8, color.z as u8];
            
            let intersect = sphere.intersect(ray);
            if intersect >= 0.0 {
                final_color = [255, 0, 0];
            };

            *image.get_pixel_mut(x, y) = image::Rgb(final_color);
        }
    }

    let elapsed = now.elapsed().as_secs_f64();
    let fps: f64 = 1.0 / elapsed;
    println!("One frame took {elapsed} seconds. That's {fps} FPS!");
    
    println!("Saving image to `output.png`...");
    image.save("output.png").unwrap();
    println!("Image saved successfully! Exiting now.");
}
