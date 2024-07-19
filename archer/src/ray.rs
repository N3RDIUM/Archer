use nalgebra::{Point3, Vector3};

// Ray struct
#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Point3<f32>,
    pub direction: Vector3<f32>,
}

impl Ray {
    pub fn new_empty() -> Ray {
        return Ray {
            origin: Point3::new(f32::NAN, f32::NAN, f32::NAN),
            direction: Vector3::new(f32::NAN, f32::NAN, f32::NAN),
        };
    }

    pub fn position_at(self, t: f32) -> Point3<f32> {
        return self.origin + t * self.direction;
    }
}
