# Imports
import math
import numpy as np

# Vectors
class Vec2(np.ndarray):
    def __new__(cls, x, y):
        return np.array([x, y], dtype=np.float32).view(cls)
    
class Vec3(np.ndarray):
    def __new__(cls, x, y, z):
        return np.array([x, y, z], dtype=np.float32).view(cls)
    
# Matrices
Matrix = np.array

# Ray class
class Ray:
    def __init__(self, origin: Vec3, direction: Vec3) -> None:
        self.origin = origin
        self.direction = direction

# Vector operations
def dot(v1, v2):
    match (len(v1), len(v2)):
        case (2, 2): return v1[0] * v2[0] + v1[1] * v2[1]
        case (3, 3): return v1[0] * v2[0] + v1[1] * v2[1] + v1[2] * v2[2]
        case _     : raise ValueError("Vectors must be of length 2 or 3")
        
def cross(v1, v2):
    match (len(v1), len(v2)):
        case (3, 3): return Vec3(v1[1] * v2[2] - v1[2] * v2[1], v1[2] * v2[0] - v1[0] * v2[2], v1[0] * v2[1] - v1[1] * v2[0])
        case _     : raise ValueError("Vectors must be of length 3")
        
def length(v):
    return math.sqrt(dot(v, v))

def normalize(v):
    return v / length(v)
