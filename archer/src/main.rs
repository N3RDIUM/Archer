use image::{Rgb, RgbImage};
use nalgebra::Point3;
use rayon::prelude::*;
use std::sync::Mutex;
use std::time::Instant;

use archer::camera::Camera;
use archer::geometries::sphere::Sphere;
use archer::materials::diffuse::Diffuse;
use archer::materials::normal::NormalMaterial;
use archer::materials::perfect_mirror::PerfectMirror;
use archer::scene::{Scene, SceneObject};
use archer::tracer::{RenderParameters, Tracer};
use archer::vectors::{Color, PixelCoord};

fn main() {
    const RESOLUTION: PixelCoord<u32> = PixelCoord::new(1280, 720);
    const TILE: PixelCoord<u32> = PixelCoord::new(80, 80);

    let im: image::ImageBuffer<Rgb<u8>, Vec<u8>> = RgbImage::new(RESOLUTION.x, RESOLUTION.y);
    let mut camera = Camera::new(RESOLUTION);
    camera.update();

    // Add some spheres
    let material: PerfectMirror = PerfectMirror {};
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
        color: Color::new(128.0, 128.0, 256.0),
        roughness: 0.242,
        albedo: 0.742,
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
        bvh: &bvh,
    };

    let params = RenderParameters {
        max_bounces: 8,
        samples: 64,
    };

    let x: Vec<u32> = (0..(RESOLUTION.x / TILE.x)).collect();
    let y: Vec<u32> = (0..(RESOLUTION.y / TILE.y)).collect();

    let now = Instant::now();
    let imutex = Mutex::new(im.clone());

    let nothing = x.par_iter().map(|tile_x| {
        let another_nothing = y.par_iter().map(|tile_y| {
            let black: Color<f64> = Color::new(0.0, 0.0, 0.0);
            let mut pixels: Vec<Vec<Color<f64>>> =
                vec![vec![black; TILE.x as usize]; TILE.y as usize];

            // Render first
            for x in 0..(TILE.x) {
                for y in 0..(TILE.y) {
                    let pixel_coord = PixelCoord::new(tile_x * TILE.x + x, tile_y * TILE.y + y);
                    let final_color = tracer.get_pixel(&pixel_coord, &params);
                    pixels[x as usize][y as usize] = final_color;
                }
            }

            // Then wait for access and write the data to the image.
            let mut image = imutex.lock().unwrap();
            for x in 0..(TILE.x) {
                for y in 0..(TILE.y) {
                    let color = pixels[x as usize][y as usize];
                    let pixel = image.get_pixel_mut(tile_x * TILE.x + x, tile_y * TILE.y + y);
                    *pixel = Rgb([color.x as u8, color.y as u8, color.z as u8]);
                }
            }
        });

        let mut sth: Vec<()> = vec![];
        another_nothing.collect_into_vec(&mut sth);
        println!("Scanline {tile_x} finished rendering.");
    });

    let mut sth: Vec<()> = vec![];
    nothing.collect_into_vec(&mut sth);
    println!("Render complete.");

    let elapsed: f64 = now.elapsed().as_secs_f64();
    let fps: f64 = 1.0 / elapsed;
    println!("One frame took {elapsed} seconds. That's {fps} FPS!");

    println!("Saving image to `output.png`...");
    let image = imutex.lock().unwrap();
    image.save("output.png").unwrap();
    println!("Image saved successfully! Exiting now.");
}
