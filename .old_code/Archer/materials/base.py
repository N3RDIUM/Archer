import taichi as ti

@ti.data_oriented
class BaseMaterial:
    """
    Base class for materials
    """
    id: str = "internal/base"
