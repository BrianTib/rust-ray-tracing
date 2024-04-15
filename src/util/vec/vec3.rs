use crate::util::{Numeric, vec::Vector, matrix::Matrix3, random_range};
//use rand::{Rng, thread_rng, distributions::uniform::SampleUniform};

#[derive(Debug, Clone, Copy)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vec3<T>
where T: Numeric
{
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }

    pub fn look_at(eye_level: Self, target: Self, up: Self) -> Matrix3<T> where T: Numeric {
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

impl<T> Vector<T> for Vec3<T>
where T: Numeric
{
    fn from(v: &[T]) -> Self {
        match v.len() {
            1 => Self { x: v[0], y: v[0], z: v[0] },
            2 => Self { x: v[0], y: v[1], z: T::zero() },
            3 => Self { x: v[0], y: v[1], z: v[2] },
            _ => Self { x: T::zero(), y: T::zero(), z: T::zero() }
        }
    }

    fn copy(other: &Self) -> Self {
        Self {
            x: other.x,
            y: other.y,
            z: other.z
        }
    }

    fn random(min: T, max: T) -> Self
    where T: Numeric
    {
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

    fn dot(&self, other: &Self) -> T where T: Numeric {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    fn squared_magnitude(&self) -> T where T: Numeric {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
}