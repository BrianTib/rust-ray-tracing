use crate::util::{Numeric, vec::Vector, random_range};

#[derive(Debug, Clone, Copy)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vec2<T>
where T: Numeric
{
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T> Vector<T> for Vec2<T>
where T: Numeric
{
    fn from(v: &[T]) -> Self {
        match v.len() {
            1 => Self { x: v[0], y: v[0] },
            2 => Self { x: v[0], y: v[1] },
            _ => Self { x: T::zero(), y: T::zero() }
        }
    }

    fn copy(other: &Self) -> Self {
        Self {
            x: other.x,
            y: other.y
        }
    }

    fn random(min: T, max: T) -> Self
    where T: Numeric
    {
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

    fn dot(&self, other: &Self) -> T where T: Numeric {
        self.x * other.x + self.y * other.y
    }
    
    fn squared_magnitude(&self) -> T where T: Numeric {
        self.x * self.x + self.y * self.y
    }
}