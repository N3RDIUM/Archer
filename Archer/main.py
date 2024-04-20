from time import perf_counter
import numpy as np
import taichi as ti
from scene import Scene
from camera import Camera
from export_image import export_image
from vectors import vec3, vec2, Color
from models.sphere import Sphere

ti.init(ti.cpu)
RESOLUTION = (1920, 1080)

scene = Scene(
    Sphere(vec3(0, 0, -5), 1),
    Color(0, 95, 95),
    4
)
camera = Camera(vec2(RESOLUTION[0], RESOLUTION[1]), vec3(0, 0, 0), vec3(0, 0, 0), 90, 0.0032)
ret = ti.field(dtype=ti.u8, shape=(camera.resolution[0], camera.resolution[1], 3))

t = perf_counter()
img = scene.render(camera, ret)
print(f"Took {perf_counter() - t} seconds. That's {1 / (perf_counter() - t)} FPS!")
print(f"Computed {img.shape[0] * img.shape[1] * scene.rpp} rays.")

render = img.to_numpy()
render = np.swapaxes(render, 0, 1)
render = np.asarray(render, dtype=np.uint8)
export_image(render, "render.png")
print("Exported image to render.png")
