use archer::camera::Camera;
use archer::vectors;
use vectors::Vec2;
use vectors::Vec3;
use vectors::Ray;

fn main() {
    let mut cam = Camera {
       resolution: Vec2 { x: 1280.0, y: 720.0 },
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

    let ray: Ray = cam.get_ray(Vec2::fill(0.0));
    println!("The direction for the top-left pixel is: {ray:?}");
    println!("I survived compilation!");
}
