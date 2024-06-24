use std::time::Instant;
use rayon::prelude::*;
use image::{RgbImage, Rgb};

use archer::geometries::sphere::Sphere;
use archer::scene_object::SceneObject;
use archer::camera::Camera;
use archer::ray::Ray;
use archer::vectors;
use vectors::Vec2;
use vectors::Vec3;

fn main() {
    const RESOLUTION: [u32; 2] = [1920, 1080];
    let white: Vec3 = Vec3::fill(255.0);
    let blue: Vec3 = Vec3 { x: 0.5 * 255.0, y: 0.7 * 255.0, z: 255.0 };

    let mut image = RgbImage::new(RESOLUTION[0], RESOLUTION[1]);
    let mut cam = Camera::new(RESOLUTION);
    cam.update();

    let sphere: Sphere = Sphere {
        radius: 0.5,
        position: Vec3 { x: 0.0, y: 0.0, z: -1.0 }
    };
    let object: SceneObject = SceneObject {
        geometry: Box::new(sphere),
    };

    let now = Instant::now();

    let pixels = image.par_enumerate_pixels_mut()
        .map(|(x, y, pixel)| {
            let ray: Ray = cam.get_ray(Vec2 { x: x as f32, y: y as f32 });
            
            let norm = ray.direction.normalize();
            let a = 0.5 * (norm.y + 1.0);
            let color: Vec3 = Vec3::fill(1.0 - a) * white + Vec3::fill(a) * blue;
            *pixel = Rgb([color.x as u8, color.y as u8, color.z as u8]);
            
            let geometry = object.geometry.as_ref();
            let intersect_point = geometry.intersect(ray);
            if !intersect_point.is_null() {
                *pixel = Rgb([255, 0, 0]);
            };
        });

    let mut sth = vec![];
    pixels.collect_into_vec(&mut sth);

    let elapsed = now.elapsed().as_secs_f64();
    let fps: f64 = 1.0 / elapsed;
    println!("One frame took {elapsed} seconds. That's {fps} FPS!");
    
    println!("Saving image to `output.png`...");
    image.save("output.png").unwrap();
    println!("Image saved successfully! Exiting now.");
}
