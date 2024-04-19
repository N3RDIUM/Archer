import taichi as ti
from vectors import vec2
from models.sphere import Sphere

ti.init(arch=ti.cpu)

@ti.dataclass
class Scene:
    sphere: Sphere

    def render(self, camera):
        ret = ti.field(dtype=ti.f32, shape=(camera.resolution[0], camera.resolution[1], 3))
        ret.fill(0)
        
        @ti.kernel
        def _render():
            for x in range(camera.resolution[0]):
                for y in range(camera.resolution[1]):
                    ray = camera.get_ray(vec2(x, y))
                    if self.sphere.intersect(ray) > 0:
                        ret[x, y, 0] = 255
                        ret[x, y, 1] = 255
                        ret[x, y, 2] = 255
        _render()

        return ret
