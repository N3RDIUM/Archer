import taichi as ti
from vectors import vec2, Color
from models.sphere import Sphere
from camera import Camera

@ti.dataclass
class Scene:
    sky: Color
    rpp: ti.u8

    def render(self, camera, objects, ret):
        sky_multiplier = ti.Vector([self.sky.r, self.sky.g, self.sky.b])
        n_objects = objects.shape[0]
        
        @ti.kernel
        def _render(camera: Camera, rpp: ti.u8):
            for x, y in ti.ndrange(int(camera.resolution[0]), int(camera.resolution[1])):
                _sumr = .0
                _sumg = .0
                _sumb = .0
                
                for pidx in range(rpp):
                    ray = camera.get_ray(vec2(x, y))
                    intersect = False
                    for obj in range(n_objects):
                        if bool(objects[obj].intersect(ray)):
                            intersect = True
                            break
                    color = ti.Vector([255, 255, 255]) * intersect + sky_multiplier * (1 - intersect)
                    
                    _sumr += color[0]
                    _sumg += color[1]
                    _sumb += color[2]
                    
                ret[x, y, 0] = ti.u8(_sumr / rpp)
                ret[x, y, 1] = ti.u8(_sumg / rpp)
                ret[x, y, 2] = ti.u8(_sumb / rpp)

        _render(camera, self.rpp)
        return ret