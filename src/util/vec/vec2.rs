use crate::util::{vec::Vector, random_range};

#[derive(Debug, Clone, Copy)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

impl Vector for Vec2 {
    fn from(v: &[f32]) -> Self {
        match v.len() {
            1 => Self { x: v[0], y: v[0] },
            2 => Self { x: v[0], y: v[1] },
            _ => Self { x: 0.0, y: 0.0 }
        }
    }

    fn copy(other: &Self) -> Self {
        Self {
            x: other.x,
            y: other.y
        }
    }

    fn random(min: f32, max: f32) -> Self {
        Self {
            x: random_range(min..max),
            y: random_range(min..max)
        }
    }

    /* OPERATIONS */
    fn add(&self, other: &Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }

    fn sub(&self, other: &Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y
        }
    }

    fn mul(&self, other: &Self) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y
        }
    }

    fn div(&self, other: &Self) -> Self {
        Self {
            x: self.x / other.x,
            y: self.y / other.y
        }
    }

    /* VECTOR FUNCTIONS */
    fn cross(&self, other: &Self) -> Self {
        Self {
            x: self.x * other.y,
            y: self.y * other.x
        }
    }

    fn dot(&self, other: &Self) -> f32 {
        self.x * other.x + self.y * other.y
    }
    
    fn squared_magnitude(&self) -> f32 {
        self.x * self.x + self.y * self.y
    }
}