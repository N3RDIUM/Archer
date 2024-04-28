# Imports
from camera import Camera
from vectors import *
c = Camera(
    Vec3(0, 0, 0),
    Vec3(0, 0, 0),
    Vec2(640, 480),
    60.0,
    0.01
)
c.cook_rays()