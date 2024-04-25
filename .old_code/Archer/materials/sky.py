import taichi as ti
from vectors import vec3, Color, Ray, normalize
from materials.base import BaseMaterial

@ti.func
def interpolate(t: ti.f32, a: Color, b: Color):
    return a * (1 - t) + b * t

@ti.data_oriented
class Sky(BaseMaterial):
    id: str = "internal/diffuse"

    @ti.func
    def color(self, previous_color: Color, ray: Ray, reflected: Ray, hit_point: vec3, normal: vec3) -> Color:
        """
        Get the color of the sky based on the ray direction.
        
        """
        t = (normalize(ray.direction)[1] + 1.0) * 0.5
        return interpolate(t, Color(255, 255, 255), Color(0, 0, 255))
