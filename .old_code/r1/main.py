import taichi as ti
from scene import Scene, Object
from camera import Camera
from vectors import Color, vec3, vec2
from materials import DiffuseMaterial
from geometries import Sphere
from renderer import Renderer

ti.init(arch=ti.gpu, unrolling_limit=0)

scene = Scene()

obj = Object(
    geometry=Sphere(vec3(0, 0, -1), 0.5),
    material=DiffuseMaterial(Color(255, 255, 255))
)

camera = Camera(
    vec2(800, 600),
    vec3(0, 0, 0),
    vec3(0, 0, 0),
    45,
    0.003
)

renderer = Renderer()
renderer.render(camera, scene, rays_per_pixel=10, max_bounces=4)
