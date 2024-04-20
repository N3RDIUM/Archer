from random import random
from time import perf_counter
import taichi as ti
from scene import Scene
from camera import Camera
from vectors import vec3, vec2, Color
from models.sphere import Sphere

ti.init(ti.gpu)
RESOLUTION = (1920, 1080)

spheres = Sphere.field(shape=1)
spheres[0].center = vec3(0, 0, -5)
spheres[0].color = Color(
    (random() + 1) / 2 * 255,
    (random() + 1) / 2 * 255,
    (random() + 1) / 2 * 255
)
spheres[0].radius = 1

scene = Scene(
    Color(0, 170, 160),
    8
)
camera = Camera(vec2(RESOLUTION[0], RESOLUTION[1]), vec3(0, 0, 0), vec3(0, 0, 0), 45, 0.0008)
ret = ti.field(dtype=ti.u8, shape=(camera.resolution[0], camera.resolution[1], 3))

gui = ti.GUI("Archer", res=RESOLUTION)
while gui.running:
    t = perf_counter()
    img = scene.render(camera, spheres, ret)
    gui.set_image(img)
    gui.show()
    print(f"\rRender took {perf_counter() - t}s ({1 / (perf_counter() - t)} FPS, {img.shape[0] * img.shape[1] * scene.rpp} rays)", end="")
