import taichi as ti
from vectors import vec2, Color
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
                    intersect_depth = 0.
                    intersect_color = Color(0, 0, 0)
                    for obj in range(n_objects):
                        i = objects[obj].intersect(ray)
                        if bool(i):
                            intersect = True
                            if intersect_depth > i or intersect_depth == 0:
                                intersect_depth = i
                                intersect_color = objects[obj].color
                    color = ti.Vector([intersect_color.r, intersect_color.g, intersect_color.b]) * intersect + sky_multiplier * (1 - intersect)
                    
                    _sumr += color[0]
                    _sumg += color[1]
                    _sumb += color[2]
                    
                ret[x, y, 0] = ti.u8(_sumr / rpp)
                ret[x, y, 1] = ti.u8(_sumg / rpp)
                ret[x, y, 2] = ti.u8(_sumb / rpp)

        _render(camera, self.rpp)
        return ret