# Imports
import taichi as ti
from vectors import Ray, vec3, vec2, normalize

@ti.dataclass
class Camera:
    """
    Camera implementation for ray tracing
    """
    resolution: vec2
    position: vec3
    rotation: vec3
    fov: ti.f32
    dither: ti.f32

    @ti.func
    def get_ray(self, pixel: vec2) -> Ray:
        """
        Returns a ray from the camera to the given pixel.
        
        Args:
            pixel (vec2): The pixel coordinate to calculate the ray for.
        
        Returns:
            Ray: The calculated ray.
        """
        # Calculate aspect ratio and normalized device coordinates
        aspect_ratio = self.resolution.x / self.resolution.y
        ndc_x = (2 * pixel.x / self.resolution.x) - 1.0
        ndc_y = 1.0 - (2 * pixel.y / self.resolution.y)

        # Calculate screen coordinates
        screen_x = ndc_x * aspect_ratio * ti.tan(self.fov / 2)
        screen_y = ndc_y * ti.tan(self.fov / 2)

        # Calculate direction vector
        direction = vec3(screen_x, screen_y, -1.0)
        
        # Add random offset to direction
        random_offset = vec3(ti.random(), ti.random(), ti.random()) * self.dither
        direction += random_offset
        
        # Calculate rotation matrix
        sinx = ti.sin(self.rotation.x)
        cosx = ti.cos(self.rotation.x)
        siny = ti.sin(self.rotation.y)
        cosy = ti.cos(self.rotation.y)
        sinz = ti.sin(self.rotation.z)
        cosz = ti.cos(self.rotation.z)
        
        rotation_matrix = ti.Matrix([
            [cosy * cosz,                  # Row 1
             sinx * siny * cosz - cosx * sinz,
             cosx * siny * cosz + sinx * sinz],

            [cosy * sinz,                  # Row 2
             sinx * siny * sinz + cosx * cosz,
             cosx * siny * sinz - sinx * cosz],

            [-siny,                        # Row 3
             sinx * cosy,
             cosx * cosy]
        ])

        # Apply rotation to direction
        direction = rotation_matrix @ direction
        
        # Normalize the direction
        direction = normalize(direction)
        
        # Calculate origin and return ray
        return Ray(self.position + random_offset, direction)
