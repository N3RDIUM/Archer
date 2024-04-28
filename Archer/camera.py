# Imports
from math import sin, cos, tan, radians
from random import random
from typing import List
from vectors import Ray, Matrix, Vec2, Vec3, normalize

class Camera:
    def __init__(
        self,
        position: Vec3,
        rotation: Vec3,
        resolution: Vec2,
        fov: float,
        jitter: float
    ) -> None:
        """
        Camera class for easy computation of rays
        """
        self.position = position
        self.rotation = rotation
        self.resolution = resolution
        self.fov = fov
        self.jitter = jitter
        
    def get_ray(
        self, 
        pixel: Vec2
    ) -> Ray:
        """
        Returns a ray from the camera to the given pixel.
        """
        # Calculate the aspect ratio and normalized device coordinates
        aspect_ratio = self.resolution[0] / self.resolution[1]
        ndc_x = (2 * pixel[0] / self.resolution[0]) - 1.0
        ndc_y = 1.0 - (2 * pixel[1] / self.resolution[1]) * aspect_ratio
        
        # Calculate the screen coordinates
        screen_x = ndc_x * tan(radians(self.fov) / 2.0)
        screen_y = ndc_y * tan(radians(self.fov) / 2.0)
        
        # Calculate the direction vector
        direction = normalize(Vec3(screen_x, screen_y, -1.0))
        
        # Add random offset to the direction
        random_offset = Vec3(
            self.jitter * (random() - 0.5),
            self.jitter * (random() - 0.5),
            self.jitter * (random() - 0.5)
        )
        direction += random_offset
        
        # Calculate the rotation matrix
        rotation_matrix = Matrix([
            [
                cos(self.rotation[1]) * cos(self.rotation[2]),
                sin(self.rotation[1]) * sin(self.rotation[0]) * cos(self.rotation[1]) - sin(self.rotation[2]) * cos(self.rotation[0]),
                sin(self.rotation[1]) * sin(self.rotation[2]) + sin(self.rotation[0]) * cos(self.rotation[1]) * cos(self.rotation[2])
            ],
            [
                sin(self.rotation[1]) * cos(self.rotation[2]),
                sin(self.rotation[1]) * sin(self.rotation[0]) * sin(self.rotation[1]) + cos(self.rotation[2]) * cos(self.rotation[2]),
                sin(self.rotation[1]) * cos(self.rotation[0]) * cos(self.rotation[2]) - sin(self.rotation[2]) * sin(self.rotation[0])
            ],
            [
                -sin(self.rotation[1]),
                sin(self.rotation[0]) * sin(self.rotation[1]),
                cos(self.rotation[0]) * cos(self.rotation[1])
            ]
        ])
        
        # Rotate the direction vector
        direction = normalize(rotation_matrix @ direction)
        
        # Return the ray
        return Ray(self.position, direction)

    def cook_rays(self) -> List[List[Ray]]:
        """
        Compute all rays in parallel
        """
        ret = []
        for y in range(int(self.resolution[0])):
            ret.append([])
            for x in range(int(self.resolution[1])):
                ret[y].append(self.get_ray(Vec2(x, y)))
        return ret
