import taichi as ti
from vectors import Ray, vec3, vec2

@ti.func
def normalize(v: vec3) -> vec3:
    """
    Normalizes a vector to have a length of 1.

    Args:
        v: The vector to normalize (ti.vec3).

    Returns:
        A normalized vector (ti.vec3).
    """
    mag = ti.sqrt(v.x * v.x + v.y * v.y + v.z * v.z)
    return v / mag

@ti.dataclass
class Camera:
    resolution: vec2
    position: vec3
    rotation: vec3
    fov: ti.f32
    dither: ti.f32

    @ti.func
    def get_ray(self, pixel: vec2) -> Ray:
        aspect_ratio = self.resolution.x / self.resolution.y
        ndc_x = (2 * pixel.x / self.resolution.x) - 1.0
        ndc_y = 1.0 - (2 * pixel.y / self.resolution.y)

        screen_x = ndc_x * aspect_ratio * ti.tan(self.fov / 2)
        screen_y = ndc_y * ti.tan(self.fov / 2)

        direction = vec3(screen_x, screen_y, -1.0)
        direction = direction + vec3(ti.random(), ti.random(), ti.random()) * self.dither
        
        sx, cx = ti.sin(self.rotation.x), ti.cos(self.rotation.x)
        sy, cy = ti.sin(self.rotation.y), ti.cos(self.rotation.y)
        sz, cz = ti.sin(self.rotation.z), ti.cos(self.rotation.z)

        rx = ti.Matrix([
            [1, 0, 0],
            [0, cx, -sx],
            [0, sx, cx]
        ])

        ry = ti.Matrix([
            [cy, 0, sy],
            [0, 1, 0],
            [-sy, 0, cy]
        ])

        rz = ti.Matrix([
            [cz, -sz, 0],
            [sz, cz, 0],
            [0, 0, 1]
        ])

        direction = rx @ direction
        direction = ry @ direction
        direction = rz @ direction
        direction = normalize(direction)
        
        return Ray(self.position + vec3(ti.random(), ti.random(), ti.random()) * self.dither, direction)
