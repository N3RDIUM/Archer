import taichi as ti
from vectors import Ray

@ti.data_oriented
class Object:
    """
    Stores a scene object as a geometry and a material
    """
    def __init__(self, geometry, material):
        self.geometry = geometry
        self.material = material

@ti.data_oriented
class Scene:
    """
    Scene class for ray tracing.
    This just stores the objects in the scene.
    Later, I will implement a `sample` func
    with BVH or octree in this class.
    """
    def __init__(self):
        self.objects = []
        self.objects_len = 0
        
    def add(self, obj: Object):
        self.objects.append(obj)
        self.objects_len += 1

    @ti.func
    def intersect(self, ray: Ray) -> list:
        ret_results = []
        ret_objs = []
        
        # TODO: These complicated functions are not @ti.func but native python
        # TODO: Compile a list of objects and rays 
        # TODO: ALlow the operations to be done in parallel in a ti.kernel
        
        for idx in ti.static(range(self.objects_len)):
            intersection = self.objects[idx].geometry.intersect(ray)
            if intersection >= 0:
                ret_results.append(intersection)
                ret_objs.append(idx)
        
        nearest_dist = None
        nearest_obj = 0
        
        if len(ret_results) > 0:
            nearest_dist = ret_results[0]
            nearest_obj = ret_objs[0]
            
            for idx in ti.static(range(len(ret_results))):
                if ret_results[idx] < nearest_dist:
                    nearest_dist = ret_results[idx]
                    nearest_obj = ret_objs[idx]
                
        return nearest_obj, nearest_dist
