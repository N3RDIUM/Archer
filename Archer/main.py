# Imports
import taichi as ti
from math import sin
from scene import Scene
from random import random
from camera import Camera
from time import perf_counter
from models.sphere import Sphere
from vectors import vec3, vec2, Color

# Initialize Taichi with the GPU
# If no GPU is detected, it automatically falls back to CPU
ti.init(ti.gpu)

# Define the resolution of the output image
RESOLUTION = (1920, 1080)

# Create a field of Sphere objects
spheres = Sphere.field(shape=1)

# Set the center and radius of the first sphere
spheres[0].center = vec3(0, 0, -5)  # Center
spheres[0].radius = 1  # Radius

# Set the color of the first sphere using random values
spheres[0].color = Color(
    (random() + 1) / 2 * 255,  # Red
    (random() + 1) / 2 * 255,  # Green
    (random() + 1) / 2 * 255   # Blue
)

# Create a Scene object with a background color and samples per pixel
scene = Scene(
    Color(0, 170, 160),  # Background color
    8  # Samples per pixel
)

# Create a Camera object with a resolution, position, rotation, and field of view
camera = Camera(
    vec2(RESOLUTION[0], RESOLUTION[1]),  # Resolution
    vec3(0, 0, 0),  # Position
    vec3(0, 0, 0),  # Rotation
    45,  # Field of view
    0.0008  # Dither
)

# Create a field for the rendered image
ret = ti.field(dtype=ti.u8, shape=(camera.resolution[0], camera.resolution[1], 3))

# Create a GUI window
gui = ti.GUI("Archer", res=RESOLUTION)

# Main rendering loop
while gui.running:
    # Start the rendering timer
    t = perf_counter()
    
    # Render the scene with the camera and spheres
    img = scene.render(camera, spheres, ret)
    
    # Print the rendering time and the number of rays traced
    print(f"\rRender took {perf_counter() - t}s ({1 / (perf_counter() - t)} FPS, {img.shape[0] * img.shape[1] * scene.rpp} rays)", end="")
    
    # Update the GUI with the rendered image
    t = perf_counter()
    gui.set_image(img)
    gui.show()
    print(f"; set_image took {perf_counter() - t}s", end="")
    
    # Update the field of view of the camera based on the current time
    camera.fov = 45 + sin(perf_counter()) / 10
