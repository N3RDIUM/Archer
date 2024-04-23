# Imports
import taichi as ti
from camera import Camera
from vectors import vec2, Color

@ti.dataclass
class Scene:
    """
    Scene class which does the actual ray tracing
    """
    sky: Color
    rpp: ti.u8

    def render(self, camera, objects, ret):
        """
        Render the scene
        """
        sky_multiplier = ti.Vector([self.sky.r, self.sky.g, self.sky.b])
        rpp = self.rpp
        
        @ti.kernel
        def _render():
            """
            The kernel function which does the actual ray tracing.
            This function renders the scene by tracing rays from the camera and
            calculating the color of each pixel.

            Args:
                camera (Camera): The camera object used to generate rays.
                rpp (ti.u8): The number of rays to trace per pixel.
            """
            # Iterate over each pixel in the image
            for x, y in ti.ndrange(int(camera.resolution[0]), int(camera.resolution[1])):
                _sumr = .0  # Red color component
                _sumg = .0  # Green color component
                _sumb = .0  # Blue color component
                
                # Loop over each sample per pixel
                ti.loop_config(parallelize=True)
                for pidx in range(rpp):
                    # Generate a ray from the camera to the pixel
                    ray = camera.get_ray(vec2(x, y))
                    
                    intersect = False  # Flag to indicate if a ray intersects with an object
                    nearest_depth = 0.  # Depth of intersection
                    nearest_color = Color(0, 0, 0)  # Color of the intersected object
                    
                    # Loop over each object in the scene to find intersection
                    ti.loop_config(parallelize=True)
                    for obj in ti.static(range(len(objects))):
                        # Get the intersection point of the ray and the object
                        i = objects[obj].intersect(ray)
                        
                        # If an intersection is found
                        if int(i) >= 0:
                            intersect = True
                            # Update the intersection if the current intersection is closer
                            if nearest_depth > i or nearest_depth == 0:
                                nearest_depth = i
                                nearest_color = objects[obj].color
                    
                    # Calculate the color of the pixel based on intersection
                    color = ti.Vector([nearest_color.r, nearest_color.g, nearest_color.b]) * intersect + sky_multiplier * (1 - intersect)
                    
                    # Accumulate the color component of each pixel
                    _sumr += color[0]
                    _sumg += color[1]
                    _sumb += color[2]
                
                # Store the average color of the pixel
                ret[x, y, 0] = ti.u8(_sumr / rpp)
                ret[x, y, 1] = ti.u8(_sumg / rpp)
                ret[x, y, 2] = ti.u8(_sumb / rpp)

        # Render the scene and return the image
        _render()
        return ret
