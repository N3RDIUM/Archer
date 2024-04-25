# Imports
import taichi as ti
from scene import Scene
from camera import Camera
from vectors import vec2, Ray, Color

class Renderer:
    def __init__(self) -> None:
        """
        This is the renderer which does the actual raytracing.
        """
    
    def render(
        self, 
        pixels: ti.lang.field,
        scene: Scene,
        camera: Camera
    ) -> None:
        """
        Renders the image
        """
        rays = Ray.field(shape=(camera.resolution[0], camera.resolution[1]))
        
        @ti.kernel
        def batch_cook_rays():
            for x in range(int(camera.resolution[0])):
                for y in range(int(camera.resolution[1])):
                    ray = camera.get_ray(vec2(x, y))
                    rays[x, y] = ray
        batch_cook_rays()
        
        intersected = scene.batch_intersect(rays)
        
        @ti.kernel
        def batch_render():
            for i in range(int(camera.resolution[0] * camera.resolution[1])):
                x = int(i % camera.resolution[0])
                y = int(i // camera.resolution[0])
                if intersected[x, y] != -1:
                    pixels[x, y, 0] = scene.objects[intersected[x, y]].material.color.x
        batch_render()
