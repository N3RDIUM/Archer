import taichi as ti
from vectors import vec3, Color, Ray, normalize
from materials.base import BaseMaterial

@ti.func
def random_in_unit_sphere() -> vec3:
    return normalize(vec3(ti.random(), ti.random(), ti.random()))

@ti.data_oriented
class SolidMaterial(BaseMaterial):
    id: str = "internal/solid" # TODO! Remove these
    
    def __init__(self, color: Color = Color(255, 255, 255)):
        self.color = color
    
    @ti.func
    def color(self, previous_color: Color, ray: Ray, reflected: Ray, hit_point: vec3, normal: vec3) -> Color:
        """
        Get the new color of the ray after reflection.
        """
        return self.color
