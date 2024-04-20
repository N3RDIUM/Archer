from time import perf_counter
import numpy as np
import taichi as ti
from scene import Scene
from camera import Camera
from export_image import export_image
from vectors import vec3, vec2, Color
from models.sphere import Sphere

ti.init(arch=ti.cpu)
RESOLUTION = (1920, 1080)

scene = Scene(
    Sphere(vec3(0, 0, -5), 1),
    Color(0, 95, 95),
    16
)
camera = Camera(vec2(RESOLUTION[0], RESOLUTION[1]), vec3(0, 0, 0), vec3(0, 0, 0), 90, 0.0032)

t = perf_counter()
img = scene.render(camera)
print(f"Took {perf_counter() - t} seconds. That's {1 / (perf_counter() - t)} FPS!")

render = img.to_numpy()
render = np.average(render, axis=2)
render = np.swapaxes(render, 0, 1)
render = np.asarray(render, dtype=np.uint8)
export_image(render, "render.png")
