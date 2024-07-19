use image::{Rgb, RgbImage};
use rayon::prelude::*;
use std::time::Instant;

use archer::camera::Camera;
use archer::geometries::sphere::Sphere;
use archer::hitinfo::HitInfo;
use archer::materials::solid::SolidColor;
use archer::ray::Ray;
use archer::scene_object::SceneObject;
use archer::vectors;
use nalgebra::Point3;
use vectors::{Color, PixelCoord};

fn main() {
    const RESOLUTION: PixelCoord<u32> = PixelCoord::new(1920, 1080);
    const MAX_BOUNCES: u32 = 1;
    const SAMPLES: u32 = 1;

    let mut image = RgbImage::new(RESOLUTION.x, RESOLUTION.y);
    let mut cam = Camera::new(RESOLUTION);
    cam.update();

    // Add a red sphere
    let sphere: Sphere = Sphere {
        radius: 0.5,
        position: Point3::new(0.0, 0.0, -1.0),
    };
    let red_mtl: SolidColor = SolidColor {
        color: Color::new(255.0, 0.0, 0.0),
    };

    let object: SceneObject = SceneObject {
        geometry: Box::new(sphere),
        material: Box::new(red_mtl),
    };

    // TODO: Move this to tracer.rs
    let now = Instant::now();
    let pixels = image.par_enumerate_pixels_mut().map(|(x, y, pixel)| {
        let mut samples = 0;
        let mut final_color: Color<f32> = Color::new(0.0, 0.0, 0.0);

        loop {
            if samples > SAMPLES {
                break;
            }

            let mut hit_info: Vec<HitInfo> = vec![];
            let mut bounces = 0;
            let ray: Ray = cam.get_ray(PixelCoord::new(x, y));
            let mut current_ray = ray.clone();

            loop {
                if bounces >= MAX_BOUNCES {
                    break;
                };

                let geometry = object.geometry.as_ref();
                let material = object.material.as_ref();

                let (hit_point, normal) = geometry.intersect(current_ray);
                if !f32::is_nan(hit_point.x * hit_point.y * hit_point.z) {
                    let bounced = material.bounce(ray, hit_point, normal);
                    current_ray = bounced;

                    let hit = HitInfo {
                        incoming: ray,
                        hit_point,
                        normal,
                        bounced,
                        geometry: Box::new(geometry),
                        material: Box::new(material),
                    };
                    hit_info.push(hit);
                }

                bounces += 1;
            }

            for hit in hit_info.iter() {
                let material = hit.material.as_ref();
                let color = material.add_color(hit.incoming, hit.hit_point, hit.normal);

                final_color.x = final_color.x + color.x;
                final_color.y = final_color.y + color.y;
                final_color.z = final_color.z + color.z;
            }
            samples += 1;
        }

        *pixel = Rgb([
            (final_color.x / samples as f32) as u8,
            (final_color.y / samples as f32) as u8,
            (final_color.z / samples as f32) as u8,
        ]);
    });

    let mut sth = vec![];
    pixels.collect_into_vec(&mut sth);

    let elapsed = now.elapsed().as_secs_f32();
    let fps: f32 = 1.0 / elapsed;
    println!("One frame took {elapsed} seconds. That's {fps} FPS!");

    println!("Saving image to `output.png`...");
    image.save("output.png").unwrap();
    println!("Image saved successfully! Exiting now.");
}
