# Imports
import taichi as ti
from vectors import Ray, vec2, vec3, normalize

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
    def get_ray(self, pixel: vec2, sinx: ti.f32, cosx: ti.f32, siny: ti.f32, cosy: ti.f32, sinz: ti.f32, cosz: ti.f32) -> Ray:
        """
        Returns a ray from the camera to the given pixel.
        
        Args:
            pixel (vec2): The pixel coordinate to calculate the ray for.
            sinx (ti.f32): The sine of the camera rotation around the X axis.
            cosx (ti.f32): The cosine of the camera rotation around the X axis.
            siny (ti.f32): The sine of the camera rotation around the Y axis.
            cosy (ti.f32): The cosine of the camera rotation around the Y axis.
            sinz (ti.f32): The sine of the camera rotation around the Z axis.
        
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

    def cook_rays(self):
        """
        Compute all rays at once.
        # TODO: Lazy-cook camera rays only if the position changes (in renderer, in a separate thread)
        """
        ret = Ray.field(shape=(self.resolution.x, self.resolution.y))
        
        @ti.kernel
        def kernel():
            sinx = ti.sin(self.rotation.x)
            cosx = ti.cos(self.rotation.x)
            siny = ti.sin(self.rotation.y)
            cosy = ti.cos(self.rotation.y)
            sinz = ti.sin(self.rotation.z)
            cosz = ti.cos(self.rotation.z)
            
            ti.loop_config(parallelize=True)
            for i, j in ret:
                ret[i, j] = self.get_ray(vec2(i, j), sinx, cosx, siny, cosy, sinz, cosz)
        
        kernel()
        return ret
