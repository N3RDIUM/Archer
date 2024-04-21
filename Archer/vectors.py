# Imports
import taichi as ti
vec3 = ti.math.vec3
vec2 = ti.math.vec2

@ti.dataclass
class Ray:
    origin: vec3
    direction: vec3
    
@ti.dataclass
class Color:
    r: ti.f32
    g: ti.f32
    b: ti.f32

@ti.func
def normalize(v: vec3) -> vec3:
    """
    Normalizes a vector to have a length of 1.

    Args:
        v: The vector to normalize (ti.vec3).

    Returns:
        A normalized vector (ti.vec3).
    """
    mag = ti.sqrt(v.x * v.x + v.y * v.y + v.z * v.z)
    return v / mag
