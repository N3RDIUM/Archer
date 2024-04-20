import taichi as ti
from vectors import vec2, Ray
from models.sphere import Sphere

@ti.dataclass
class Scene:
    sphere: Sphere
    rpp: ti.u8

    def render(self, camera):
        ret = ti.field(dtype=ti.u8, shape=(camera.resolution[0], camera.resolution[1], self.rpp, 3))
        rays = Ray.field(shape=(camera.resolution[0], camera.resolution[1], self.rpp))
        
        @ti.kernel
        def _render():
            for pidx in range(self.rpp):
                for x, y in ti.ndrange(int(camera.resolution[0]), int(camera.resolution[1])):
                    rays[x, y, pidx] = camera.get_ray(vec2(x, y))
                    
            for pidx in range(self.rpp):
                ti.loop_config(parallelize=True)
                for x, y in ti.ndrange(int(camera.resolution[0]), int(camera.resolution[1])):
                    ret[x, y, pidx, 0] = ti.u8(255 * (self.sphere.intersect(rays[x, y, pidx]) > 0))
                    ret[x, y, pidx, 1] = ti.u8(255 * (self.sphere.intersect(rays[x, y, pidx]) > 0))
                    ret[x, y, pidx, 2] = ti.u8(255 * (self.sphere.intersect(rays[x, y, pidx]) > 0))
                    
        _render()
        return ret
