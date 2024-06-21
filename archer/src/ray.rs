use crate::vectors::Vec3;

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
