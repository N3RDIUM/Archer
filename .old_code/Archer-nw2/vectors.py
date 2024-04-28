# Imports
import taichi as ti

# 2d, 3d, and 4d vectors
vec2 = ti.math.vec2
vec3 = ti.math.vec3
vec4 = ti.math.vec4

# Color dataclass
Color = ti.math.vec3

# Ray dataclass
@ti.dataclass
class Ray:
    origin: vec3
    direction: vec3

# Normalize function
@ti.func
def normalize(v: vec3) -> vec3:
    """
    Normalizes a vector.
    """
    magnitude = ti.sqrt(v.x**2 + v.y**2 + v.z**2)
    return vec3(v.x / magnitude, v.y / magnitude, v.z / magnitude)
