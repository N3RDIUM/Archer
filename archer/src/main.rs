use nalgebra::Point3;
use rayon::prelude::*;
use std::time::Instant;
use image::{Rgb, RgbImage};

use archer::camera::Camera;
use archer::geometries::sphere::Sphere;
use archer::scene::{Scene, SceneObject};
use archer::vectors::{PixelCoord, Color};
use archer::materials::diffuse::Diffuse;
use archer::materials::normal::NormalMaterial;
use archer::tracer::{RenderParameters, Tracer};
use archer::materials::perfect_mirror::PerfectMirror;

fn main() {
    const RESOLUTION: PixelCoord<u32> = PixelCoord::new(640, 480);
    let mut image: image::ImageBuffer<Rgb<u8>, Vec<u8>> = RgbImage::new(RESOLUTION.x, RESOLUTION.y);
    let mut camera = Camera::new(RESOLUTION);
    camera.update();

    // Add some spheres
    let material = PerfectMirror {};
    let sphere: Sphere = Sphere {
        radius: 1.0,
        position: Point3::new(1.0, 0.0, -4.0),
    };
    let ball: SceneObject = SceneObject {
        geometry: Box::new(sphere),
        material: Box::new(material),
        node_index: 0,
    };

    let material1 = NormalMaterial {};
    let sphere1: Sphere = Sphere {
        radius: 1.0,
        position: Point3::new(-1.0, 0.0, -4.0),
    };
    let otherball: SceneObject = SceneObject {
        geometry: Box::new(sphere1),
        material: Box::new(material1),
        node_index: 0,
    };

    let ground_mtl = Diffuse {
        color: Color::new(200.0, 200.0, 200.0),
        roughness: 0.24,
        albedo: 0.74
    };
    let ground_geom = Sphere {
        radius: 1000.0,
        position: Point3::new(0.0, -1001.0, 0.0),
    };
    let ground = SceneObject {
        geometry: Box::new(ground_geom),
        material: Box::new(ground_mtl),
        node_index: 0,
    };

    // Create the scene
    let mut scene: Scene = Scene { objects: vec![] };
    scene.add(ball);
    scene.add(otherball);
    scene.add(ground);
    
    let bvh = scene.build_bvh();

    // Finally, make the tracer and let the magic happen!
    let tracer: Tracer = Tracer {
        scene: &scene,
        camera: &camera,
        bvh: &bvh
    };

    let params = RenderParameters {
        max_bounces: 16,
        samples: 128
    };
    
    let now = Instant::now();
    let pixels = image.par_enumerate_pixels_mut().map(|(x, y, pixel)| {
        let pixel_coord = PixelCoord::new(x as u32, y as u32);
        let final_color = tracer.get_pixel(&pixel_coord, &params);

        *pixel = Rgb([
            final_color.x as u8,
            final_color.y as u8,
            final_color.z as u8,
        ]);
    });

    let mut sth: Vec<()> = vec![];
    pixels.collect_into_vec(&mut sth);

    let elapsed: f64 = now.elapsed().as_secs_f64();
    let fps: f64 = 1.0 / elapsed;
    println!("One frame took {elapsed} seconds. That's {fps} FPS!");

    println!("Saving image to `output.png`...");
    image.save("output.png").unwrap();
    println!("Image saved successfully! Exiting now.");
}
