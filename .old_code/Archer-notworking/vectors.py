# Imports
import cython
import numpy as np

@cython.cclass
class Vec2:
    x = cython.declare(cython.float, visibility="public")
    y = cython.declare(cython.float, visibility="public")
    
    def __init__(self, x, y):
        self.x = x
        self.y = y
    
    def length(self):
        return (self.x**2 + self.y**2)**0.5
    
    def normalize(self):
        length = self.length()
        if length == 0:
            return Vec2(0, 0)
        else:
            return Vec2(self.x / length, self.y / length)
        
    def dot(self, other):
        assert isinstance(other, Vec2)
        return self.x * other.x + self.y * other.y
    
    def to_numpy(self):
        return np.array([self.x, self.y])
    
    def __getitem__(self, index):
        if index == 0:
            return self.x
        elif index == 1:
            return self.y
        else:
            raise IndexError("Index out of range")
        
    def __len__(self):
        return 2
    
    def __repr__(self):
        return f"Vec2({self.x}, {self.y})"
    
    def __add__(self, other):
        if isinstance(other, Vec2):
            return Vec2(self.x + other.x, self.y + other.y)
        elif isinstance(other, float):
            return Vec2(self.x + other, self.y + other)
        raise TypeError("Unsupported operand type(s) for +: 'Vec2' and '{}'".format(type(other)))
    
    def __sub__(self, other):
        if isinstance(other, Vec2):
            return Vec2(self.x - other.x, self.y - other.y)
        elif isinstance(other, float):
            return Vec2(self.x - other, self.y - other)
        raise TypeError("Unsupported operand type(s) for -: 'Vec2' and '{}'".format(type(other)))
    
    def __mul__(self, other):
        if isinstance(other, Vec2):
            return Vec2(self.x * other.x, self.y * other.y)
        elif isinstance(other, float):
            return Vec2(self.x * other, self.y * other)
        raise TypeError("Unsupported operand type(s) for *: 'Vec2' and '{}'".format(type(other)))
    
    def __truediv__(self, other):
        if isinstance(other, Vec2):
            return Vec2(self.x / other.x, self.y / other.y)
        elif isinstance(other, float):
            return Vec2(self.x / other, self.y / other)
        raise TypeError("Unsupported operand type(s) for /: 'Vec2' and '{}'".format(type(other)))
    
    def __eq__(self, other):
        if isinstance(other, Vec2):
            return self.x == other.x and self.y == other.y
        raise TypeError("Unsupported operand type(s) for ==: 'Vec2' and '{}'".format(type(other)))
    
    def __ne__(self, other):
        if isinstance(other, Vec2):
            return self.x != other.x or self.y != other.y
        raise TypeError("Unsupported operand type(s) for !=: 'Vec2' and '{}'".format(type(other)))
    
    def __neg__(self):
        return Vec2(-self.x, -self.y)
    
@cython.cclass
class Vec3:
    x = cython.declare(cython.float, visibility="public")
    y = cython.declare(cython.float, visibility="public")
    z = cython.declare(cython.float, visibility="public")
    
    def __init__(self, x, y, z):
        self.x = x
        self.y = y
        self.z = z
    
    def length(self):
        return (self.x**2 + self.y**2 + self.z**2)**0.5
    
    def normalize(self):
        length = self.length()
        if length == 0:
            return Vec3(0, 0, 0)
        else:
            return Vec3(self.x / length, self.y / length, self.z / length)
    
    def dot(self, other):
        assert isinstance(other, Vec3)
        return self.x * other.x + self.y * other.y + self.z * other.z
    
    def to_numpy(self):
        return np.array([self.x, self.y, self.z])
    
    def __getitem__(self, index):
        if index == 0:
            return self.x
        elif index == 1:
            return self.y
        elif index == 2:
            return self.z
        else:
            raise IndexError("Index out of range")
        
    def __len__(self):
        return 3
    
    def __repr__(self):
        return f"Vec3({self.x}, {self.y}, {self.z})"
    
    def __add__(self, other):
        if isinstance(other, Vec3):
            return Vec3(self.x + other.x, self.y + other.y, self.z + other.z)
        elif isinstance(other, float):
            return Vec3(self.x + other, self.y + other, self.z + other)
        raise TypeError("Unsupported operand type(s) for +: 'Vec3' and '{}'".format(type(other)))
    
    def __sub__(self, other):
        if isinstance(other, Vec3):
            return Vec3(self.x - other.x, self.y - other.y, self.z - other.z)
        elif isinstance(other, float):
            return Vec3(self.x - other, self.y - other, self.z - other)
        raise TypeError("Unsupported operand type(s) for -: 'Vec3' and '{}'".format(type(other)))
    
    def __mul__(self, other):
        if isinstance(other, Vec3):
            return Vec3(self.x * other.x, self.y * other.y, self.z * other.z)
        elif isinstance(other, float):
            return Vec3(self.x * other, self.y * other, self.z * other)
        raise TypeError("Unsupported operand type(s) for *: 'Vec3' and '{}'".format(type(other)))
    
    def __truediv__(self, other):
        if isinstance(other, Vec3):
            return Vec3(self.x / other.x, self.y / other.y, self.z / other.z)
        elif isinstance(other, float):
            return Vec3(self.x / other, self.y / other, self.z / other)
        raise TypeError("Unsupported operand type(s) for /: 'Vec3' and '{}'".format(type(other)))
    
    def __eq__(self, other):
        if isinstance(other, Vec3):
            return self.x == other.x and self.y == other.y and self.z == other.z
        raise TypeError("Unsupported operand type(s) for ==: 'Vec3' and '{}'".format(type(other)))
    
    def __ne__(self, other):
        if isinstance(other, Vec3):
            return self.x != other.x or self.y != other.y or self.z != other.z
        raise TypeError("Unsupported operand type(s) for !=: 'Vec3' and '{}'".format(type(other)))
    
    def __neg__(self):
        return Vec3(-self.x, -self.y, -self.z)
    
@cython.cclass
class PixelCoord(Vec2): pass
@cython.cclass
class Position(Vec3): pass
@cython.cclass
class Direction(Vec3): pass
@cython.cclass
class Color(Vec3): pass
@cython.cclass
class Ray:
    origin = cython.declare(Vec3, visibility="public")
    direction = cython.declare(Vec3, visibility="public")
    
    def __init__(self, origin: Vec3, direction: Vec3):
        self.origin = origin
        self.direction = direction

@cython.cclass
class Matrix:
    matrix = cython.declare(np.ndarray, visibility="public")
    
    def __init__(self, matrix: np.ndarray):
        self.matrix = matrix
    
    def __getitem__(self, index):
        return self.matrix[index]
    
    def __len__(self):
        return len(self.matrix)

    def __repr__(self):
        return f"Matrix({self.matrix})"

    def __mul__(self, other):
        if isinstance(other, Matrix):
            return Matrix(self.matrix @ other.matrix)
        elif isinstance(other, Vec3):
            m = self.matrix @ other.to_numpy()
            return Vec3(m[0], m[1], m[2])
        raise TypeError("Unsupported operand type(s) for *: 'Matrix' and '{}'".format(type(other)))
    
    def __eq__(self, other):
        if isinstance(other, Matrix):
            return np.array_equal(self.matrix, other.matrix)
        raise TypeError("Unsupported operand type(s) for ==: 'Matrix' and '{}'".format(type(other)))
    
    def __ne__(self, other):
        if isinstance(other, Matrix):
            return not np.array_equal(self.matrix, other.matrix)
        raise TypeError("Unsupported operand type(s) for !=: 'Matrix' and '{}'".format(type(other)))

    def __neg__(self):
        return Matrix(-self.matrix)

    def __add__(self, other):
        if isinstance(other, Matrix):
            return Matrix(self.matrix + other.matrix)
        raise TypeError("Unsupported operand type(s) for +: 'Matrix' and '{}'".format(type(other)))

    def __sub__(self, other):
        if isinstance(other, Matrix):
            return Matrix(self.matrix - other.matrix)
        raise TypeError("Unsupported operand type(s) for -: 'Matrix' and '{}'".format(type(other)))

    def __truediv__(self, other):
        if isinstance(other, Matrix):
            return Matrix(self.matrix / other.matrix)
        raise TypeError("Unsupported operand type(s) for /: 'Matrix' and '{}'".format(type(other)))

    def __matmul__(self, other):
        if isinstance(other, Matrix):
            return Matrix(self.matrix @ other.matrix)
        elif isinstance(other, Vec3):
            m = self.matrix @ other.to_numpy()
            return Vec3(m[0], m[1], m[2])
        raise TypeError("Unsupported operand type(s) for *: 'Matrix' and '{}'".format(type(other)))

    def __pow__(self, other):
        if isinstance(other, int):
            return Matrix(self.matrix ** other)
        raise TypeError("Unsupported operand type(s) for **: 'Matrix' and '{}'".format(type(other)))
