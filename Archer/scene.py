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
        
        sky_multiplier = ti.Vector([self.sky.r, self.sky.g, self.sky.b]) * 255
        
        @ti.kernel
        def _render(camera: Camera, sphere: Sphere, rpp: ti.u8):
            ti.loop_config(parallelize=True)
            for x, y in ti.ndrange(int(camera.resolution[0]), int(camera.resolution[1])):
                ti.loop_config(parallelize=True)
                for pidx in range(rpp):
                    ray = camera.get_ray(vec2(x, y))
                    intersect = sphere.intersect(ray)
                    hit_color = ti.Vector([0, 0, 0])
                    
                    if intersect > 0:
                        hit_color = ti.Vector([255, 255, 255])
                        
                    ret[x, y, pidx, 0] = ti.u8(hit_color[0] + sky_multiplier[0] * (intersect <= 0))
                    ret[x, y, pidx, 1] = ti.u8(hit_color[1] + sky_multiplier[1] * (intersect <= 0))
                    ret[x, y, pidx, 2] = ti.u8(hit_color[2] + sky_multiplier[2] * (intersect <= 0))

        _render(camera, self.sphere, self.rpp)
        return ret