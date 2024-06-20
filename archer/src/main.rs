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

    for x in 0..RESOLUTION.x as i32 {
        for y in 0..RESOLUTION.y as i32{
            let _r: Ray = do_it(&cam, x, y);

            // I could not believe that it was so fast
            // So I just decided to print stuff out temporarily
            // Yes, it is as fast as it runs.
            // println!("Ray {_r:?}");
        }
    }

    println!("I survived compilation!");
}
