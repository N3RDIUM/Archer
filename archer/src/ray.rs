use crate::types::{Position, Direction};

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Position<f64>,
    pub direction: Direction<f64>
}

impl Ray {
    pub fn new() -> Ray {
        return Ray {
            origin: Position::<f64>::origin(),
            direction: Direction::<f64>::zeros(),
        }
    }

    pub fn at(self, t: f64) -> Position<f64> {
        return self.origin + self.direction * t;
    }
}

