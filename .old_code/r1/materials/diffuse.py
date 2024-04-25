import taichi as ti
from vectors import vec3, Color, Ray, normalize
from materials.base import BaseMaterial

@ti.func
def random_in_unit_sphere() -> vec3:
    return normalize(vec3(ti.random(), ti.random(), ti.random()))

@ti.data_oriented
class DiffuseMaterial(BaseMaterial):
    id: str = "internal/diffuse"
    
    def __init__(self, color: Color = Color(255, 255, 255)):
        self.color = color

    @ti.func
    def bounce(self, ray: Ray, hit_point: vec3, normal: vec3) -> Ray:
        """
        Bounce the ray off the surface.
        """
        # Scatter the ray in a random direction
        direction = random_in_unit_sphere()
        
        # If the ray is coming from the inside, flip the direction
        if ti.dot(normal, ray.direction) > 0:
            direction = -direction
        
        return Ray(origin=hit_point, direction=direction)
    
    @ti.func
    def color(self, previous_color: Color, ray: Ray, reflected: Ray, hit_point: vec3, normal: vec3) -> Color:
        """
        Get the new color of the ray after reflection.
        """
        return Color(255, 255, 255) # For now
