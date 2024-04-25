# Imports
import taichi as ti
from vectors import Ray, Color, vec3

@ti.dataclass
class Sphere:
    """
    Sphere hittable class
    """
    center: vec3
    radius: ti.f32

    @ti.func
    def intersect(self, ray: Ray) -> ti.f32:
        """
        Intersect function to check if the ray goes through the sphere.

        Args:
            ray (Ray): The ray to check intersection with.

        Returns:
            ti.f32: The distance at which the ray intersects the sphere, or -1 if no intersection.
        """
        # Calculate the parameters of the quadratic equation representing the sphere intersection.
        oc = ray.origin - self.center  # Offset vector from the center of the sphere to the ray origin.
        a = ti.math.dot(ray.direction, ray.direction)  # Squared length of the ray direction.
        b = 2.0 * ti.math.dot(oc, ray.direction)  # Dot product of the offset and ray direction.
        c = ti.math.dot(oc, oc) - self.radius * self.radius  # Squared distance between the ray origin and the sphere center.

        # Calculate the discriminant of the quadratic equation.
        d = b * b - 4 * a * c

        # Check if the ray intersects the sphere.
        # If the discriminant is negative, no intersection occurs.
        # If it's zero, the ray touches the sphere at one point.
        # If it's positive, the ray touches the sphere at two points.
        # The smaller root is the nearest intersection point, so we return that one.
        ret = -1.
        if d >= 0:
            ret = (-b - ti.sqrt(d)) / (2.0 * a)
        return ret # The distance at which the ray intersects the sphere, or -1 if no intersection.
