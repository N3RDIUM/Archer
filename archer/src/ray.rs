use crate::vectors::Vec3;

// Ray struct
#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new_empty() -> Ray {
        return Ray {
            origin: Vec3::new_empty(),
            direction: Vec3::new_empty(),
        };
    }

    pub fn position_at(self, t: f64) -> Vec3 {
        return self.origin + Vec3::fill(t) * self.direction;
    }
}
