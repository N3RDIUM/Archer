import taichi as ti

@ti.data_oriented
class BaseGeometry:
    """
    Base class for geometries
    """
    id: str = "internal/base"
