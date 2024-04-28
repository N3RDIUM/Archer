# Imports
import cython
import numpy as np
from random import random
from math import sin, cos, tan
from vectors import Vec3, Ray, PixelCoord, Matrix

@cython.cclass
class Camera:
    position = cython.declare(Vec3, visibility="public")
    rotation = cython.declare(Vec3, visibility="public")
    fov = cython.declare(cython.float, visibility="public")
    resolution = cython.declare(PixelCoord, visibility="public")
    dither = cython.declare(cython.float, visibility="public")
    
    def __init__(
        self,
        position: Vec3,
        rotation: Vec3,
        fov: cython.float,
        resolution: PixelCoord,
        dither: cython.float
    ):
        self.position = position
        self.rotation = rotation
        self.fov = fov
        self.resolution = resolution
        self.dither = dither
    
    @cython.exceptval(check=False)
    def get_ray(self, x: cython.float, y: cython.float) -> Vec3:
        # Calculate aspect ratio and normalized device coordinates
        aspect_ratio = self.resolution.x / self.resolution.y
        ndc_x = 2 * x / self.resolution.x - 1
        ndc_y = 1 - 2 * y / self.resolution.y
        
        # Calculate screen coordinates
        screen_x = ndc_x * aspect_ratio * tan(self.fov / 2)
        screen_y = ndc_y * tan(self.fov / 2)
        
        # Calculate ray direction
        direction = Vec3(screen_x, screen_y, -1).normalize()
        
        # Add a random offset to the ray direction
        random_offset = Vec3(random(), random(), random()) * self.dither
        direction += random_offset
        
        # Calculate some trigonometric ratios
        sinx = sin(self.rotation.x)
        cosx = cos(self.rotation.x)
        siny = sin(self.rotation.y)
        cosy = cos(self.rotation.y)
        sinz = sin(self.rotation.z)
        cosz = cos(self.rotation.z)
        
        # Compute the rotation matrix
        matrix = Matrix(np.array([
            [cosy * cosz, -cosx * sinz + sinx * siny * cosz, sinx * sinz + cosx * siny * cosz],
            [cosy * sinz, cosx * cosz + sinx * siny * sinz, -sinx * cosz + cosx * siny * sinz],
            [-siny, sinx * cosy, cosx * cosy]
        ]))
        
        # Rotate the ray direction and normalize it
        direction = matrix @ direction
        direction = direction.normalize()
        
        # Return the ray
        return Ray(self.position, direction)

    @cython.exceptval(check=False)
    def cook_rays(self) -> list[Ray]:
        rays = []
        for x in range(self.resolution.x):
            for y in range(self.resolution.y):
                rays.append(self.get_ray(x, y))
        return rays