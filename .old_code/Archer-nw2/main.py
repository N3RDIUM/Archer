# Imports
import taichi as ti
from vectors import *
from camera import Camera
from config import RESOLUTION
from time import perf_counter
from scene import Scene, Object
from geometries.sphere import Sphere
from geometries.infinite_plane import InfinitePlane

# Initialize ti and the GUI
ti.init(arch=ti.gpu, default_fp=ti.f32, default_ip=ti.i32)
gui = ti.GUI("Archer", res=RESOLUTION)
pixels = ti.field(dtype=ti.f32, shape=[RESOLUTION[0], RESOLUTION[1], 3])

# Initialize the camera, renderer and scene
scene = Scene()
camera = Camera(
    resolution=vec2(RESOLUTION),
    position=vec3(0, 0, 10),
    rotation=vec3(0, 0, 0),
    fov=45,
    dither=0.01
)

# Add a sphere
sphere = Object()
sphere.set_geometry(Sphere(position=vec3(0, 0, 0), radius=1), Sphere)
scene.add_object(sphere)

# Add an infinite plane
plane = Object()
plane.set_geometry(InfinitePlane(position=vec3(0, -5, 0), rotation=vec3(0, 0, 0)), InfinitePlane)
scene.add_object(plane)

# Add random objects
for i in range(100):
    obj = Object()
    obj.set_geometry(Sphere(position=vec3(ti.random(), ti.random(), ti.random()) * 5, radius=1), Sphere)
    scene.add_object(obj)

# Cook the scene
t = perf_counter()
rays = camera.cook_rays()
intersections = scene.batch_intersect(rays)
print(f"Render took {perf_counter() - t} secs. That's {1 / (perf_counter() - t)} FPS!")

# Render
gui.set_image(pixels)
while gui.running: gui.show()
