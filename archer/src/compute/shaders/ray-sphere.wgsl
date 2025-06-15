struct Ray {
    origin: vec3<f32>,
    _pad1: f32,
    dir: vec3<f32>,
    _pad2: f32,
};

struct Sphere {
    center: vec3<f32>,
    radius: f32,
};

@group(0) @binding(0) var<storage, read> ray: Ray;
@group(0) @binding(1) var<storage, read> sphere: Sphere;
@group(0) @binding(2) var<storage, read_write> result: f32;

@compute @workgroup_size(1)
fn main() {
    let oc = ray.origin - sphere.center;
    let a = dot(ray.dir, ray.dir);
    let b = 2.0 * dot(oc, ray.dir);
    let c = dot(oc, oc) - sphere.radius * sphere.radius;
    let discriminant = b * b - 4.0 * a * c;

    if discriminant < 0.0 {
        result = -1.0;
    } else {
        result = (-b - sqrt(discriminant)) / (2.0 * a);
    }
}
