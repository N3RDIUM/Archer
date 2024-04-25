# Imports
import taichi as ti
from vectors import Ray
    
@ti.data_oriented
class Object:
    def __init__(self) -> None:
        """
        Object implementation for ray tracing
        """
        self.id = None
        self.geometry = None
        self.material = None

@ti.data_oriented
class Scene:
    def __init__(self) -> None:
        """
        Scene implementation for ray tracing
        """
        self.objects = {}
        
    def add(self, obj) -> str:
        """
        Adds an object to the scene
        """
        id = len(list(self.objects.values()))
        self.objects[id] = obj
        obj.id = id
        return id

    def remove(self, id) -> None:
        """
        Removes an object from the scene
        """
        del self.objects[id]

    def batch_intersect(self, rays):
        results = ti.field(dtype=ti.i32, shape=rays.shape)
        objects = []
        for obj in self.objects.values():
            if obj.geometry: objects.append(obj)
        n_objects = len(objects)
        
        @ti.kernel
        def batch_intersect():
            for x in range(rays.shape[0]):
                for y in range(rays.shape[1]):
                    nearest = 1e10
                    res = -1
                    
                    for i in ti.static(range(n_objects)):
                        ray = rays[x, y]
                        
                        dist = objects[i].geometry.intersect(ray)
                        if dist < nearest:
                            nearest = dist
                            res = objects[i].id
                        
                    results[x, y] = res

        batch_intersect()
        return results
