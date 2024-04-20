from time import perf_counter
import numpy as np
import taichi as ti
from scene import Scene
from camera import Camera
from export_image import export_image
from vectors import vec3, vec2, Color
from models.sphere import Sphere

ti.init(ti.gpu, default_fp=ti.f32, default_ip=ti.i32)
RESOLUTION = (1920, 1080)

spheres = Sphere.field(shape=5)
spheres[0].center = vec3(0, 0, -5)
spheres[0].radius = 1
spheres[1].center = vec3(0, -4, -5)
spheres[1].radius = 1
spheres[2].center = vec3(4, 0, -5)
spheres[2].radius = 1
spheres[3].center = vec3(-4, 0, -5)
spheres[3].radius = 1
spheres[4].center = vec3(0, 4, -5)
spheres[4].radius = 1

scene = Scene(
    Color(0, 95, 95),
    4
)
camera = Camera(vec2(RESOLUTION[0], RESOLUTION[1]), vec3(0, 0, 0), vec3(0, 0, 0), 90, 0.0032)
ret = ti.field(dtype=ti.u8, shape=(camera.resolution[0], camera.resolution[1], 3))

t = perf_counter()
img = scene.render(camera, spheres, ret)
print(f"Took {perf_counter() - t} seconds. That's {1 / (perf_counter() - t)} FPS!")
print(f"Computed {img.shape[0] * img.shape[1] * scene.rpp} rays.")

render = img.to_numpy()
render = np.swapaxes(render, 0, 1)
render = np.asarray(render, dtype=np.uint8)
export_image(render, "render.png")
print("Exported image to render.png")
