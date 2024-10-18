extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::pixels::PixelFormatEnum;
use std::time::Duration;

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
use archer::vectors::{ColorVector, PixelCoord};

fn main() -> Result<(), String> {
    const RESOLUTION: PixelCoord<u32> = PixelCoord::new(1280, 720);
    const TILE: PixelCoord<u32> = PixelCoord::new(4, 4);

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Archer", RESOLUTION.x, RESOLUTION.y)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();

    let mut texture = texture_creator
        .create_texture_streaming(PixelFormatEnum::RGB24, RESOLUTION.x, RESOLUTION.y)
        .map_err(|e| e.to_string())?;

    let mut event_pump = sdl_context.event_pump().unwrap();

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
                color: ColorVector::new(128.0, 128.0, 256.0),
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
        samples: 8,
    };

    let tiles_x = (0..(RESOLUTION.x / TILE.x)).collect::<Vec<_>>();
    let tiles_y = (0..(RESOLUTION.y / TILE.y)).collect::<Vec<_>>();

    'running: loop {
        let now = Instant::now();

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running Ok(()),
                _ => {}
            }
        }

        let pixels = vec![
            vec![ColorVector::new(0.0, 0.0, 0.0); RESOLUTION.y as usize];
            RESOLUTION.x as usize
        ];
        let pixels_mutex = Mutex::new(pixels.clone());

        tiles_x.par_iter().for_each(|tile_x| {
            tiles_y.par_iter().for_each(|tile_y| {
                let mut tile =
                    vec![vec![ColorVector::new(0.0, 0.0, 0.0); TILE.y as usize]; TILE.x as usize];

                // Render pixels in the tile
                for x in 0..TILE.x {
                    for y in 0..TILE.y {
                        let pixel_coord = PixelCoord::new(tile_x * TILE.x + x, tile_y * TILE.y + y);
                        tile[x as usize][y as usize] = tracer.get_pixel(&pixel_coord, &params);
                    }
                }

                // Write pixels to the image
                let mut pixels_locked = pixels_mutex.lock().unwrap();
                for x in 0..TILE.x {
                    for y in 0..TILE.y {
                        pixels_locked[(tile_x * TILE.x + x) as usize]
                            [(tile_y * TILE.y + y) as usize] = tile[x as usize][y as usize];
                    }
                }
            });
        });

        texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
            let pixels_locked = pixels_mutex.lock().unwrap();

            for x in 0..(RESOLUTION.x as usize) {
                for y in 0..(RESOLUTION.y as usize) {
                    let offset = y * pitch + x * 3;
                    buffer[offset] = pixels_locked[x][y].x as u8;
                    buffer[offset + 1] = pixels_locked[x][y].y as u8;
                    buffer[offset + 2] = pixels_locked[x][y].z as u8;
                }
            }
        })?;

        canvas.clear();
        canvas.copy(&texture, None, None)?;
        canvas.present();

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));

        let elapsed = now.elapsed().as_secs_f64();
        let fps = 1.0 / elapsed;
        println!("Render complete in {elapsed} seconds. That's {fps} FPS!");
    }
}
