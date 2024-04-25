import taichi as ti
from vectors import vec3, Ray
from geometries.base import BaseGeometry

@ti.data_oriented
class Sphere(BaseGeometry):
    """
    A sphere geometry
    """
    id: str = "internal/sphere"
    
    def __init__(self, position: vec3 = vec3(0, 0, 0), radius: ti.f32 = 1, color: vec3 = vec3(255, 255, 255)):
        self.position = position
        self.radius = radius
        self.color = color

    @ti.func
    def intersect(self, ray: Ray) -> ti.f32:
        """
        Intersects a ray with the sphere
        """
        # Calculate the vector from the sphere's center to the ray's origin
        oc = ray.origin - self.position

        # Calculate the discriminant
        a = ti.math.dot(ray.direction, ray.direction)
        b = 2 * ti.math.dot(oc, ray.direction)
        c = ti.math.dot(oc, oc) - self.radius**2
        discriminant = b**2 - 4 * a * c

        # If the discriminant is negative, there are no real roots
        t1 = -1.0
        t2 = -1.0
        if discriminant >= 0:
            # Calculate the two intersection points
            t1 = (-b - ti.sqrt(discriminant)) / (2 * a)
            t2 = (-b + ti.sqrt(discriminant)) / (2 * a)

        # Return the closest intersection point
        return min(t1, t2)
