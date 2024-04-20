import taichi as ti
from vectors import Ray, Color, vec3

@ti.dataclass
class Sphere:
    center: vec3
    radius: ti.f32
    color: Color

    @ti.func
    def intersect(self, ray: Ray) -> ti.f32:
        oc = ray.origin - self.center
        a = ti.math.dot(ray.direction, ray.direction)
        b = 2.0 * ti.math.dot(oc, ray.direction)
        c = ti.math.dot(oc, oc) - self.radius * self.radius
        d = b * b - 4 * a * c
        
        return (-b - ti.sqrt(d)) / (2.0 * a) * d >= 0