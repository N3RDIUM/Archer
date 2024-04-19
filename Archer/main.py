from time import perf_counter
import numpy as np
from scene import Scene
from camera import Camera
from export_image import export_image
from vectors import vec3, vec2
from models.sphere import Sphere

RESOLUTION = (1920, 1080)

scene = Scene(
    Sphere(vec3(0, 0, -5), 1),
)
camera = Camera(vec2(RESOLUTION[0], RESOLUTION[1]), vec3(0, 0, 0), vec3(0, 0, 0), 60)

t = perf_counter()
img = scene.render(camera)
print(f"Took {perf_counter() - t} seconds. That's {1 / (perf_counter() - t)} FPS!")

render = np.asarray(img.to_numpy(), dtype=np.uint8)
export_image(render, "render.png")
