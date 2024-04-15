use crate::util::{vec::Vector, matrix::Matrix3, random_range};
//use rand::{Rng, thread_rng, distributions::uniform::SampleUniform};

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn look_at(eye_level: Self, target: Self, up: Self) -> Matrix3 {
        // Calculate the forward, right, and up vectors
        let forward = target.sub(&eye_level).normalize();
        let right = forward.cross(&up.normalize());
        let up = right.cross(&forward);

        Matrix3::new([
            [right.x, right.y, right.z],
            [up.x, up.y, up.z],
            [-forward.x, -forward.y, -forward.z]
        ])
    }
}

impl Vector for Vec3 {
    fn from(v: &[f32]) -> Self {
        match v.len() {
            1 => Self { x: v[0], y: v[0], z: v[0] },
            2 => Self { x: v[0], y: v[1], z: 0.0 },
            3 => Self { x: v[0], y: v[1], z: v[2] },
            _ => Self { x: 0.0, y: 0.0, z: 0.0 }
        }
    }

    fn copy(other: &Self) -> Self {
        Self {
            x: other.x,
            y: other.y,
            z: other.z
        }
    }

    fn random(min: f32, max: f32) -> Self {
        Self {
            x: random_range(min..max),
            y: random_range(min..max),
            z: random_range(min..max)
        }
    }

    /* OPERATIONS */
    fn add(&self, other: &Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }

    fn sub(&self, other: &Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
        }
    }

    fn mul(&self, other: &Self) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z
        }
    }

    fn div(&self, other: &Self) -> Self {
        Self {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z
        }
    }

    /* VECTOR FUNCTIONS */
    fn cross(&self, other: &Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    fn dot(&self, other: &Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    fn squared_magnitude(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
}