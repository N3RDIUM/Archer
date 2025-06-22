struct CameraParams {
    resolution: vec2<u32>,
    _pad0: vec2<u32>,
    position: vec3<f32>,
    _pad1: f32,
    focal_length: f32,
    viewport_height: f32,
    _pad2: vec2<f32>,
};

struct Ray {
    origin: vec3<f32>,
    _pad1: f32,
    direction: vec3<f32>,
    _pad2: f32,
};

@group(0) @binding(0)
var<storage, read> params: CameraParams;

@group(0) @binding(1)
var<storage, read_write> rays: array<Ray>;

@compute @workgroup_size(16, 16)
fn main(@builtin(global_invocation_id) gid: vec3<u32>) {
    let width = params.resolution.x;
    let height = params.resolution.y;

    if (gid.x >= width || gid.y >= height) {
        return;
    }

    // Normalized coordinates in [-1, 1]
    let u = (f32(gid.x) + 0.5) / f32(width) * 2.0 - 1.0;
    let v = (f32(gid.y) + 0.5) / f32(height) * 2.0 - 1.0;

    // Viewport width derived from height and aspect ratio
    let aspect_ratio = f32(width) / f32(height);
    let viewport_width = aspect_ratio * params.viewport_height;

    // Ray direction in camera space
    let dir = normalize(vec3<f32>(
        u * viewport_width * 0.5,
        -v * params.viewport_height * 0.5,
        -params.focal_length
    ));

    let index = gid.y * width + gid.x;
    rays[index] = Ray(params.position, 0.0, dir, 0.0);
}

