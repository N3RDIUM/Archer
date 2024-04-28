import taichi as ti
from vectors import vec3, Ray

@ti.dataclass
class InfinitePlane:
    """
    An infinite plane geometry
    """
    position: vec3
    rotation: vec3

    @ti.func
    def intersect(self, ray: Ray) -> ti.f32:
        """
        Intersects a ray with the plane
        """
        # Calculate the dot product between the ray's direction and the plane's normal
        denom = ti.math.dot(ray.direction, self.rotation)
        
        # If the dot product is zero, the ray is parallel to the plane and does not intersect
        ret = -1.0

        # Calculate the distance to the plane
        dist = -ti.math.dot(self.position - ray.origin, self.rotation) / denom

        # If the distance is negative, the ray is intersecting the plane behind the ray origin
        if dist >= 0.0:
            ret = dist

        # Return the distance to the plane
        return dist

    @ti.func
    def normal(self, point: vec3) -> vec3:
        """
        Return the normal which is uniform to the plane
        """
        return self.rotation
