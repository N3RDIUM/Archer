use bvh::ray::Ray as BvhRay;
use nalgebra::{Point3, Vector3};

// Ray struct
#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Point3<f64>,
    pub direction: Vector3<f64>,
}

impl Ray {
    pub fn new_empty() -> Self {
        Ray {
            origin: Point3::new(f64::NAN, f64::NAN, f64::NAN),
            direction: Vector3::new(f64::NAN, f64::NAN, f64::NAN),
        }
    }

    pub fn position_at(self, t: f64) -> Point3<f64> {
        self.origin + t * self.direction
    }

    pub fn to_bvh_ray(self) -> BvhRay<f64, 3> {
        BvhRay::new(self.origin, self.direction)
    }
}
