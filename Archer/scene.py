import taichi as ti
from vectors import vec2, Color
from models.sphere import Sphere
from camera import Camera

@ti.dataclass
class Scene:
    sphere: Sphere
    sky: Color
    rpp: ti.u8

    def render(self, camera, ret):
        sky_multiplier = ti.Vector([self.sky.r, self.sky.g, self.sky.b])
        
        @ti.kernel
        def _render(camera: Camera, sphere: Sphere, rpp: ti.u8):
            for x, y in ti.ndrange(int(camera.resolution[0]), int(camera.resolution[1])):
                _sumr = .0
                _sumg = .0
                _sumb = .0
                
                for pidx in range(rpp):
                    ray = camera.get_ray(vec2(x, y))
                    intersect = sphere.intersect(ray)
                    hit = intersect > 0
                    color = ti.Vector([255, 255, 255]) * hit + sky_multiplier * (1 - hit)
                    
                    _sumr += color[0]
                    _sumg += color[1]
                    _sumb += color[2]
                    
                ret[x, y, 0] = ti.u8(_sumr / rpp)
                ret[x, y, 1] = ti.u8(_sumg / rpp)
                ret[x, y, 2] = ti.u8(_sumb / rpp)

        _render(camera, self.sphere, self.rpp)
        return ret