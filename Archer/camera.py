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
        random_offset = vec3(ti.random(), ti.random(), ti.random()) * self.dither
        direction += random_offset

        rotation_matrix = ti.Matrix([
            [ti.cos(self.rotation.y) * ti.cos(self.rotation.z),
             ti.sin(self.rotation.x) * ti.sin(self.rotation.y) * ti.cos(self.rotation.z) - ti.cos(self.rotation.x) * ti.sin(self.rotation.z),
             ti.cos(self.rotation.x) * ti.sin(self.rotation.y) * ti.cos(self.rotation.z) + ti.sin(self.rotation.x) * ti.sin(self.rotation.z)],

            [ti.cos(self.rotation.y) * ti.sin(self.rotation.z),
             ti.sin(self.rotation.x) * ti.sin(self.rotation.y) * ti.sin(self.rotation.z) + ti.cos(self.rotation.x) * ti.cos(self.rotation.z),
             ti.cos(self.rotation.x) * ti.sin(self.rotation.y) * ti.sin(self.rotation.z) - ti.sin(self.rotation.x) * ti.cos(self.rotation.z)],

            [-ti.sin(self.rotation.y),
             ti.sin(self.rotation.x) * ti.cos(self.rotation.y),
             ti.cos(self.rotation.x) * ti.cos(self.rotation.y)]
        ])

        direction = rotation_matrix @ direction
        direction = normalize(direction)
        
        return Ray(self.position + random_offset, direction)