use nalgebra::Point3;
use rayon::prelude::*;
use std::time::Instant;
use image::{Rgb, RgbImage};

use archer::camera::Camera;
use archer::geometries::sphere::Sphere;
use archer::scene::{Scene, SceneObject};
use archer::vectors::PixelCoord;
use archer::materials::normal::NormalMaterial;
use archer::tracer::{RenderParameters, Tracer};

fn main() {
    const RESOLUTION: PixelCoord<u32> = PixelCoord::new(1920, 1080);
    let mut image: image::ImageBuffer<Rgb<u8>, Vec<u8>> = RgbImage::new(RESOLUTION.x, RESOLUTION.y);
    let mut camera = Camera::new(RESOLUTION);
    camera.update();

    // Add some spheres
    let material = NormalMaterial {};
    let sphere: Sphere = Sphere {
        radius: 1.0,
        position: Point3::new(0.0, 0.0, -1.42),
    };
    let object: SceneObject = SceneObject {
        geometry: Box::new(sphere),
        material: Box::new(material),
        node_index: 0,
    };

    // Create the scene
    let mut scene: Scene = Scene { objects: vec![] };
    scene.add(object);
    
    let bvh = scene.build_bvh();

    // Finally, make the tracer and let the magic happen!
    let tracer: Tracer = Tracer {
        scene: &scene,
        camera: &camera,
        bvh: &bvh
    };

    let params = RenderParameters {
        max_bounces: 4,
        samples: 32
    };

    // TODO: Split screen space into boxes or scanlines and trace!!
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

    let elapsed: f32 = now.elapsed().as_secs_f32();
    let fps: f32 = 1.0 / elapsed;
    println!("One frame took {elapsed} seconds. That's {fps} FPS!");

    println!("Saving image to `output.png`...");
    image.save("output.png").unwrap();
    println!("Image saved successfully! Exiting now.");
}
