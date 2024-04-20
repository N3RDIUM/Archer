import taichi as ti
from vectors import vec2
from models.sphere import Sphere

@ti.dataclass
class Scene:
    sphere: Sphere
    rpp: ti.u8

    def render(self, camera):
        ret = ti.field(dtype=ti.u8, shape=(camera.resolution[0], camera.resolution[1], self.rpp, 3))
        
        @ti.kernel
        def _render():
            ti.loop_config(parallelize=True)
            for pidx in range(self.rpp):
                ti.loop_config(parallelize=True)
                for x, y in ti.ndrange(int(camera.resolution[0]), int(camera.resolution[1])):
                    ray = camera.get_ray(vec2(x, y))
                    if self.sphere.intersect(ray) > 0:
                        ret[x, y, pidx, 0] = ti.u8(255)
                        ret[x, y, pidx, 1] = ti.u8(255)
                        ret[x, y, pidx, 2] = ti.u8(255)
                    
        _render()
        return ret
