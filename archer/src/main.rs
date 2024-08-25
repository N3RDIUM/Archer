use image::{Rgb, RgbImage};
use rayon::prelude::*;
use std::rc::Rc;
use std::time::Instant;

use archer::camera::Camera;
use archer::geometries::sphere::Sphere;
use archer::geometries::base::Geometry;
use archer::hitinfo::HitInfo;
use archer::materials::solid::SolidColor;
use archer::materials::base::Material;
use archer::ray::Ray;
use archer::scene::{Scene, SceneObject};
use archer::vectors;
use nalgebra::Point3;
use vectors::{Color, PixelCoord};

fn get_current_ray(ray: &Ray, hit_info: &Vec<HitInfo>) -> Ray {
    if hit_info.len() > 0 {
        return hit_info[hit_info.len() - 1].bounced.clone()
    } else {
        return ray.clone()
    }
}

fn main() {
    const RESOLUTION: PixelCoord<u32> = PixelCoord::new(1920, 1080);
    const MAX_BOUNCES: u32 = 1;
    const SAMPLES: u32 = 12;

    let white: Color<f32> = Color::new(255.0, 254.0, 253.0);
    let blue: Color<f32> = Color::new(0.5 * 255.0, 0.7 * 255.0, 255.0);

    let mut image: image::ImageBuffer<Rgb<u8>, Vec<u8>> = RgbImage::new(RESOLUTION.x, RESOLUTION.y);
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
        node_index: 0,
    };

    // Create the scene
    let mut scene: Scene = Scene { objects: vec![] };
    scene.add(object);
    let bvh = scene.build_bvh();

    // TODO: Move this to tracer.rs
    // TODO: Split screen space into boxes or scanlines and trace!!
    let now = Instant::now();
    let pixels = image.par_enumerate_pixels_mut().map(|(x, y, pixel)| {
        let mut samples: u32 = 0;
        let mut final_color: Color<f32> = Color::new(0.0, 0.0, 0.0);

        loop {
            if samples > SAMPLES {
                break;
            }

            let mut hit_info: Vec<HitInfo> = vec![];
            let mut bounces: u32 = 0;
            let ray: Ray = cam.get_ray(PixelCoord::new(x, y));

            loop {
                if bounces >= MAX_BOUNCES {
                    break;
                };
                let current_ray = Rc::new(get_current_ray(&ray, &hit_info));
                let intersections: Vec<&Box<SceneObject>> = scene.intersect(&bvh, Rc::clone(&current_ray));

                if intersections.len() == 0 {
                    break;
                }

                let nearest: &SceneObject = intersections[0].as_ref();
                let geometry: &Box<dyn Geometry + Send + Sync> = &nearest.geometry;
                let material: &Box<dyn Material + Send + Sync> = &nearest.material;

                let (hit_point, normal) = geometry.intersect(&current_ray);
                if !f32::is_nan(hit_point.x * hit_point.y * hit_point.z) {
                    let previous = current_ray.clone();
                    let new = material.bounce(&current_ray, hit_point, normal);

                    let hit = HitInfo {
                        incoming: *previous,
                        hit_point,
                        normal,
                        bounced: new,
                        object: Box::new(nearest),
                    };

                    hit_info.push(hit);
                }

                bounces += 1;
            }
            
            if hit_info.len() == 0 {
                let norm = ray.direction.normalize();
                let a = 0.5 * (norm.y + 1.0);
                final_color.x += white.x * (1.0 - a) + blue.x * a;
                final_color.y += white.y * (1.0 - a) + blue.y * a;
                final_color.z += white.z * (1.0 - a) + blue.z * a;
            }

            for hit in hit_info.iter() {
                let hit_info: &HitInfo = hit.to_owned();
                let object: &SceneObject = hit_info.object.as_ref();
                let color = object.material.add_color(&hit.incoming, hit.hit_point, hit.normal);

                final_color.x += color.x;
                final_color.y += color.y;
                final_color.z += color.z;
            }
            
            samples += 1;
        }

        *pixel = Rgb([
            (final_color.x / samples as f32) as u8,
            (final_color.y / samples as f32) as u8,
            (final_color.z / samples as f32) as u8,
        ]);
    });

    let mut sth: Vec<()> = vec![];
    pixels.collect_into_vec(&mut sth);

    let elapsed: f32 = now.elapsed().as_secs_f32();
    let fps: f32 = 1.0 / elapsed;
    println!("One frame took {elapsed} seconds. That's {fps} FPS!");

    println!("Saving image to `output.png`...");
    image.save("output.png").unwrap();
    println!("Image saved successfully! Exiting now.");
}
