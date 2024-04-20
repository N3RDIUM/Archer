import taichi as ti
from vectors import vec2, Ray
from models.sphere import Sphere

@ti.dataclass
class Scene:
    sphere: Sphere
    rpp: ti.u8

    def render(self, camera):
        ret = ti.field(dtype=ti.u8, shape=(camera.resolution[0], camera.resolution[1], self.rpp, 3))
        
        @ti.kernel
        def _render():
            idx = 0
            while idx <= camera.resolution[0] * camera.resolution[1] * self.rpp:
                x = int((idx // self.rpp) % camera.resolution[0])
                y = int((idx // self.rpp) // camera.resolution[0])
                pidx = int(idx % self.rpp)
                
                ray= camera.get_ray(vec2(x, y))
                ret[x, y, pidx, 0] = ti.u8(255 * (self.sphere.intersect(ray) > 0))
                ret[x, y, pidx, 1] = ti.u8(255 * (self.sphere.intersect(ray) > 0))
                ret[x, y, pidx, 2] = ti.u8(255 * (self.sphere.intersect(ray) > 0))
                
                idx += 1
                
        _render()
        return ret
