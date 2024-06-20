use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;

// Vector structs
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32
}

impl Add for Vec2 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

impl Sub for Vec2 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y
        }
    }
}

impl Mul for Vec2 {
    type Output = Self;

    fn mul(self, other:Self) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y
        }
    }
}

impl Div for Vec2 {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Self {
            x: self.x / other.x,
            y: self.y / other.y
        }
    }
}

impl Vec2 {
    pub fn fill(value: f32) -> Vec2 {
        Vec2 {
            x: value,
            y: value
        }
    }

    pub fn length(&self) -> f32 {
        let len_squared: f32 = self.x * self.x + self.y * self.y;
        return len_squared.sqrt()
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
        }
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, other:Self) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z
        }
    }
}

impl Div for Vec3 {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Self {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z
        }
    }
}

impl Vec3 {
    pub fn fill(value: f32) -> Vec3 {
        Vec3 {
            x: value,
            y: value,
            z: value
        }
    }

    pub fn length(&self) -> f32 {
        let len_squared: f32 = self.x * self.x + self.y * self.y + self.z * self.z;
        return len_squared.sqrt();
    }
}

// Ray struct
#[derive(Debug)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3
}

impl Ray {
    pub fn position_at(self, t: f32) -> Vec3 {
        return self.origin + Vec3::fill(t) * self.direction;
    }
}
