struct CameraParams {
    resolution: vec2<u32>,
    position: vec3<f32>,
    _pad1: f32,
    focal_length: f32,
    viewport_height: f32,
    _pad2: vec2<f32>,
};

struct GPURay {
    origin: vec4<f32>,
    dir: vec4<f32>,
};

@group(0) @binding(0)
var<uniform> camera: CameraParams;

@group(0) @binding(1)
var<storage, read_write> ray_buffer: array<GPURay>;

@compute @workgroup_size(8, 8, 1)
fn main(@builtin(global_invocation_id) id: vec3<u32>) {
    let x = id.x;
    let y = id.y;

    if x >= camera.resolution.x || y >= camera.resolution.y {
        return;
    }

    let res = vec2<f32>(camera.resolution);
    let aspect = res.x / res.y;
    let viewport_width = camera.viewport_height * aspect;

    let viewport_u = vec3<f32>(viewport_width, 0.0, 0.0);
    let viewport_v = vec3<f32>(0.0, -camera.viewport_height, 0.0);
    let delta_u = viewport_u / res.x;
    let delta_v = viewport_v / res.y;

    let top_left = camera.position
        - vec3<f32>(0.0, 0.0, -camera.focal_length)
        - viewport_u * 0.5
        - viewport_v * 0.5
        + (delta_u + delta_v) * 0.5;

    let pixel_center = top_left + delta_u * f32(x) + delta_v * f32(y);
    let ray_dir = normalize(pixel_center - camera.position);

    let index = y * camera.resolution.x + x;

    ray_buffer[index].origin = vec4<f32>(camera.position, 0.0);
    ray_buffer[index].dir = vec4<f32>(ray_dir, 0.0);
}

