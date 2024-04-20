import taichi as ti
from vectors import vec2, Color
from models.sphere import Sphere
from camera import Camera

@ti.dataclass
class Scene:
    sphere: Sphere
    sky: Color
    rpp: ti.u8

    def render(self, camera):
        ret = ti.field(dtype=ti.u8, shape=(camera.resolution[0], camera.resolution[1], self.rpp, 3))
        
        @ti.kernel
        def _render(camera: Camera, sphere: Sphere, rpp: ti.u8):
            idx = 0
            while idx <= camera.resolution[0] * camera.resolution[1] * rpp:
                x = int((idx // rpp) % camera.resolution[0])
                y = int((idx // rpp) // camera.resolution[0])
                pidx = int(idx % rpp)
                
                ray= camera.get_ray(vec2(x, y))
                ret[x, y, pidx, 0] = ti.u8(255 * (sphere.intersect(ray) > 0) + self.sky.r * (sphere.intersect(ray) <= 0))
                ret[x, y, pidx, 1] = ti.u8(255 * (sphere.intersect(ray) > 0) + self.sky.g * (sphere.intersect(ray) <= 0))
                ret[x, y, pidx, 2] = ti.u8(255 * (sphere.intersect(ray) > 0) + self.sky.b * (sphere.intersect(ray) <= 0))
                
                idx += 1
        _render(camera, self.sphere, self.rpp)
        
        return ret
