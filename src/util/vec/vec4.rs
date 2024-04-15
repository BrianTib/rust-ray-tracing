use crate::util::{matrix::Matrix4, vec::Vector, random_range};
//use rand::{Rng, thread_rng, distributions::uniform::SampleUniform};

#[derive(Debug, Clone, Copy)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32
}

impl Vec4 {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    pub fn look_at(eye_level: Self, target: Self, up: Self) -> Matrix4 {
        // Calculate the forward, right, and up vectors
        let forward = target.sub(&eye_level).normalize();
        let right = forward.cross(&up.normalize());
        let up = right.cross(&forward);

        Matrix4::new([
            [right.x, right.y, right.z, right.w],
            [up.x, up.y, up.z, up.w],
            [-forward.x, -forward.y, -forward.z, -forward.w],
            [0.0, 0.0, 0.0, 1.0]
        ])
    }
}

impl Vector for Vec4 {
    fn from(v: &[f32]) -> Self {
        match v.len() {
            1 => Self { x: v[0], y: v[0], z: v[0], w: v[0] },
            2 => Self { x: v[0], y: v[1], z: 0.0, w: 0.0 },
            3 => Self { x: v[0], y: v[1], z: v[2], w: 0.0 },
            4 => Self { x: v[0], y: v[1], z: v[2], w: v[3] },
            _ => Self { x: 0.0, y: 0.0, z: 0.0, w: 0.0 }
        }
    }

    fn copy(other: &Self) -> Self {
        Self {
            x: other.x,
            y: other.y,
            z: other.z,
            w: other.w
        }
    }

    fn random(min: f32, max: f32) -> Self {
        Self {
            x: random_range(min..max),
            y: random_range(min..max),
            z: random_range(min..max),
            w: random_range(min..max)
        }
    }

    /* OPERATIONS */
    fn add(&self, other: &Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w
        }
    }

    fn sub(&self, other: &Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w
        }
    }

    fn mul(&self, other: &Self) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
            w: self.w * other.w
        }
    }

    fn div(&self, other: &Self) -> Self {
        Self {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
            w: self.w / other.w
        }
    }

    /* VECTOR FUNCTIONS */
    fn cross(&self, other: &Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
            w: self.w * other.w
        }
    }

    fn dot(&self, other: &Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }

    fn squared_magnitude(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w
    }
}