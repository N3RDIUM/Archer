# Imports
from camera import Camera
from vectors import Vec3, PixelCoord

c = Camera(
    Vec3(0, 0, 0),
    Vec3(0, 0, 0),
    90,
    PixelCoord(1920, 1080),
    0.1
)
rays = c.cook_rays()