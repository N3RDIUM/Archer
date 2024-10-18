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
    const RESOLUTION: PixelCoord<u32> = PixelCoord::new(1920, 1080);
    const TILE: PixelCoord<u32> = PixelCoord::new(40, 40);

    let im: RgbImage = RgbImage::new(RESOLUTION.x, RESOLUTION.y);
    let mut camera = Camera::new(RESOLUTION);
    camera.update();

    // Create materials and spheres
    let spheres = vec![
        Box::new(SceneObject {
            geometry: Box::new(Sphere {
                radius: 1.0,
                position: Point3::new(1.0, 0.0, -4.0),
            }),
            material: Box::new(PerfectMirror {}),
            node_index: 0,
        }),
        Box::new(SceneObject {
            geometry: Box::new(Sphere {
                radius: 1.0,
                position: Point3::new(-1.0, 0.0, -4.0),
            }),
            material: Box::new(NormalMaterial {}),
            node_index: 0,
        }),
        Box::new(SceneObject {
            geometry: Box::new(Sphere {
                radius: 1000.0,
                position: Point3::new(0.0, -1001.0, 0.0),
            }),
            material: Box::new(Diffuse {
                color: Color::new(128.0, 128.0, 256.0),
                roughness: 0.242,
                albedo: 0.742,
            }),
            node_index: 0,
        }),
    ];

    // Create the scene and build BVH
    let mut scene = Scene { objects: spheres };
    let bvh = scene.build_bvh();

    // Create tracer
    let tracer = Tracer {
        scene: &scene,
        camera: &camera,
        bvh: &bvh,
    };

    let params = RenderParameters {
        max_bounces: 8,
        samples: 64,
    };

    let now = Instant::now();
    let imutex = Mutex::new(im.clone());

    let tiles_x = (0..(RESOLUTION.x / TILE.x)).collect::<Vec<_>>();
    let tiles_y = (0..(RESOLUTION.y / TILE.y)).collect::<Vec<_>>();

    tiles_x.par_iter().for_each(|tile_x| {
        tiles_y.par_iter().for_each(|tile_y| {
            let mut pixels =
                vec![vec![Color::new(0.0, 0.0, 0.0); TILE.y as usize]; TILE.x as usize];

            // Render pixels in the tile
            for x in 0..TILE.x {
                for y in 0..TILE.y {
                    let pixel_coord = PixelCoord::new(tile_x * TILE.x + x, tile_y * TILE.y + y);
                    pixels[x as usize][y as usize] = tracer.get_pixel(&pixel_coord, &params);
                }
            }

            // Write pixels to the image
            let mut image = imutex.lock().unwrap();
            for x in 0..TILE.x {
                for y in 0..TILE.y {
                    let color = pixels[x as usize][y as usize];
                    let pixel = image.get_pixel_mut(tile_x * TILE.x + x, tile_y * TILE.y + y);
                    *pixel = Rgb([color.x as u8, color.y as u8, color.z as u8]);
                }
            }

            println!("Tile ({}, {}) finished rendering.", tile_x, tile_y);
        });
    });

    let elapsed = now.elapsed().as_secs_f64();
    let fps = 1.0 / elapsed;
    println!("Render complete in {elapsed} seconds. That's {fps} FPS!");

    println!("Saving image to `output.png`...");
    let image = imutex.lock().unwrap();
    image.save("output.png").unwrap();
    println!("Image saved successfully! Exiting now.");
}
