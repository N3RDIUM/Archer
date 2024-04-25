# Imports
import taichi as ti
from camera import Camera
from renderer import Renderer
from time import perf_counter
from config import RESOLUTION
from geometries import Sphere
from vectors import vec2, vec3
from scene import Scene, Object
from materials import SolidMaterial

# Setup ti, gui, and pixel buffer
ti.init(arch=ti.gpu, default_fp=ti.f32, default_ip=ti.i32)
gui = ti.GUI("Archer", res=RESOLUTION)
pixels = ti.field(dtype=ti.f32, shape=[RESOLUTION[0], RESOLUTION[1], 3])
pixels.fill(0)

# Setup the scene, camera and renderer
renderer = Renderer()
scene = Scene()
camera = Camera(
    vec2(RESOLUTION[0], RESOLUTION[1]),
    vec3(0, 0, 0),
    vec3(0, 0, 0),
    60.0,
    0.042
)

sphere = Object()
sphere.geometry = Sphere(vec3(0, 0, -50), 1)
sphere.material = SolidMaterial(vec3(255, 255, 255))
scene.add(sphere)

sphere1 = Object()
sphere1.geometry = Sphere(vec3(0, 0, -5), 1)
sphere1.material = SolidMaterial(vec3(255, 255, 255))
scene.add(sphere1)

# Mainloop
while gui.running:
    renderer.render(pixels, scene, camera)
    gui.set_image(pixels)
    gui.show()
