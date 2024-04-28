# Imports
import taichi as ti
from uuid import uuid4
from vectors import vec3

@ti.data_oriented
class Object:
    def __init__(self) -> None:
        """
        Represents a single object in the scene.
        """
        self.id = None
        self.geometry = None
        self.geometry_dataclass = None
        self.material = None
        self.material_dataclass = None
        
    def set_geometry(self, geometry, geometry_class):
        """
        Set the geometry of the object
        """
        self.geometry = geometry
        self.geometry_dataclass = geometry_class
        
    def set_material(self, material, material_class):
        """
        Set the material of the object
        """
        self.material = material
        self.material_dataclass = material_class
        
@ti.data_oriented
class Scene:
    def __init__(self) -> None:
        """
        Scene class for ray tracing
        """
        self.objects = {}
        
    def add_object(self, obj):
        """
        Add an object to the scene
        """
        id = len(self.objects)
        obj.id = id
        self.objects[id] = obj
        return id
    
    def remove_object(self, id):
        """
        Remove an object from the scene
        """
        if id not in self.objects:
            return
        del self.objects[id]

    def batch_intersect(self, rays):
        """
        Intersect all objects in the scene in a kernel
        #TODO! Make a ti array of pointers to the objects
        #TODO! Prawblum sawlvd!
        """
        results = {}
        result_dist = {}
        n_objects = len(self.objects)
        @ti.kernel
        def kernel():
            for x, y in ti.ndrange(int(rays.shape[0]), int(rays.shape[1])):
                ray = rays[x, y]
                key = (ray.origin, ray.direction)
                for i in ti.static(range(n_objects)):
                    obj = self.objects[i]
        kernel()
        return results
