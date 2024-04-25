import taichi as ti
from scene import Scene
from camera import Camera
from vectors import vec2, vec3, Color, Ray

@ti.dataclass
class Record:
    hit_point: vec3
    normal: vec3
    bounced: Ray

@ti.data_oriented
class Renderer:
    def __init__(self):
        pass
    
    def render(
        self,
        
        camera: Camera,
        scene: Scene,
        
        rays_per_pixel: int,
        max_bounces: int
    ):
        pixels = ti.field(dtype=ti.f32, shape=(camera.resolution.x, camera.resolution.y, 3))
        record = Record.field(shape=max_bounces)

        @ti.kernel
        def render_kernel():
            for x in range(int(camera.resolution.x)):
                for y in range(int(camera.resolution.y)):
                    final = Color(0, 0, 0)
                    
                    for _ in range(rays_per_pixel):
                        pixel = vec2(x, y)
                        ray = camera.get_ray(pixel)
                        _record = []
                        
                        for bounce in range(max_bounces):
                            obj, dist = scene.intersect(ray)
                            if obj:
                                hit_point = ray.origin + ray.direction * dist
                                normal = obj.geometry.normal(hit_point)
                                bounced = obj.material.bounce(ray, hit_point, normal)
                                record[bounce] = Record(hit_point, normal, bounced)
                                _record.append(obj)
                                ray = bounced
                        
                        color = Color(0, 0, 0)
                        for b in ti.static(range(len(record), 0)):
                            bounce = record[b]
                            obj = _record[b]
                            hit_point = bounce.hit_point
                            normal = bounce.normal
                            bounced = bounce.bounced
                            
                            color = scene.objects[obj].material.color(color, ray, bounced, hit_point, normal)

                        final += color
                    
                    final /= rays_per_pixel
                    pixels[x, y, 0] = final[0]
                    pixels[x, y, 1] = final[1]
                    pixels[x, y, 2] = final[2]
                    
        render_kernel()
        return pixels
