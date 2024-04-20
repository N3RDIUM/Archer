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
