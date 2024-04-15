use crate::util::{Numeric, Float, matrix::Matrix4, vec::Vector, random_range};
//use rand::{Rng, thread_rng, distributions::uniform::SampleUniform};

#[derive(Debug, Clone, Copy)]
pub struct Vec4<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T
}

impl<T> Vec4<T>
where T: Numeric
{
    pub fn new(x: T, y: T, z: T, w: T) -> Self {
        Self { x, y, z, w }
    }

    pub fn look_at(eye_level: Self, target: Self, up: Self) -> Matrix4<T> where T: Numeric {
        // Calculate the forward, right, and up vectors
        let forward = target.sub(&eye_level).normalize();
        let right = forward.cross(&up.normalize());
        let up = right.cross(&forward);

        Matrix4::new([
            [right.x, right.y, right.z, right.w],
            [up.x, up.y, up.z, up.w],
            [-forward.x, -forward.y, -forward.z, -forward.w],
            [T::zero(), T::zero(), T::zero(), T::one()]
        ])
    }
}

impl<T> Vector<T> for Vec4<T>
where T: Numeric
{
    fn from(v: &[T]) -> Self {
        match v.len() {
            1 => Self { x: v[0], y: v[0], z: v[0], w: v[0] },
            2 => Self { x: v[0], y: v[1], z: T::zero(), w: T::zero() },
            3 => Self { x: v[0], y: v[1], z: v[2], w: T::zero() },
            4 => Self { x: v[0], y: v[1], z: v[2], w: v[3] },
            _ => Self { x: T::zero(), y: T::zero(), z: T::zero(), w: T::zero() }
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

    fn random(min: T, max: T) -> Self
    where T: Numeric
    {
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

    fn dot(&self, other: &Self) -> T where T: Float {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }

    fn squared_magnitude(&self) -> T where T: Float {
        self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w
    }
}

// impl<T> Vec4<T>
// where
//     T: Numeric,
// {
//     pub fn new(x: T, y: T, z: T, w: T) -> Self {
//         Self { x, y, z, w }
//     }

//     pub fn from(v: &[T]) -> Self {
//         match v.len() {
//             1 => Self { x: v[0], y: v[0], z: v[0], w: v[0] },
//             2 => Self { x: v[0], y: v[1], z: T::zero(), w: T::zero() },
//             3 => Self { x: v[0], y: v[1], z: v[2], w: T::zero() },
//             4 => Self { x: v[0], y: v[1], z: v[2], w: v[3] },
//             _ => Self { x: T::zero(), y: T::zero(), z: T::zero(), w: T::zero() }
//         }
//     }

//     pub fn copy(other: &Self) -> Self {
//         Self {
//             x: other.x,
//             y: other.y,
//             z: other.z,
//             w: other.w
//         }
//     }
    
//     /* ADDITION */
//     /// Returns the result of adding `other` to `self`
//     pub fn add(&self, other: &Self) -> Self {
//         Self {
//             x: self.x + other.x,
//             y: self.y + other.y,
//             z: self.z + other.z,
//             w: self.w + other.w
//         }
//     }

//     /// Adds `other` to self
//     pub fn add_mut(&mut self, other: &Self) -> &mut Self {
//         *self = self.add(other);
//         self
//     }

//     /// Add value of type `T` to all of the properties of `self`
//     pub fn add_by(&self, value: T) -> Self {
//         let other = Vec4::<T>::from(&[value]);
//         self.add(&other)
//     }

//     /// Add value of type `T` to all of the properties of `self`
//     pub fn add_by_mut(&mut self, value: T) -> &mut Self {
//         let other = Vec4::<T>::from(&[value]);
//         Self::add_mut(self, &other)
//     }

//     /* SUBSTITUTION */
//     /// Returns the result of substracting `other` to `self`
//     pub fn sub(&self, other: &Self) -> Self {
//         Self {
//             x: self.x - other.x,
//             y: self.y - other.y,
//             z: self.z - other.z,
//             w: self.w - other.w
//         }
//     }

//     /// Substracts `other` from self
//     pub fn sub_mut(&mut self, other: &Self) -> &mut Self {
//         *self = self.sub(other);
//         self
//     }

//     /// Add value of type `T` to all of the properties of `self`
//     pub fn sub_by(&self, value: T) -> Self {
//         let other = Vec4::<T>::from(&[value]);
//         self.sub(&other)
//     }

//     /// Add value of type `T` to all of the properties of `self`
//     pub fn sub_by_mut(&mut self, value: T) -> &mut Self {
//         let other = Vec4::<T>::from(&[value]);
//         Self::sub_mut(self, &other)
//     }

//     /* MULTIPLICATION */
//     /// Returns the result of multiplying `other` to `self`
//     pub fn mul(&self, other: &Self) -> Self {
//         Self {
//             x: self.x * other.x,
//             y: self.y * other.y,
//             z: self.z * other.z,
//             w: self.w * other.w
//         }
//     }

//     /// Multiplies `other` to self
//     pub fn mul_mut(&mut self, other: &Self) -> &mut Self {
//         *self = self.mul(other);
//         self
//     }

//     /// Multiply all of the properties of `self` by a singular value of type `T`
//     pub fn mul_by(&self, value: T) -> Self {
//         let other = Vec4::<T>::from(&[value]);
//         self.mul(&other)
//     }

//     /// Multiply all of the properties of `self` by a singular value of type `T`
//     pub fn mul_by_mut(&mut self, value: T) -> &mut Self {
//         let other = Vec4::<T>::from(&[value]);
//         Self::mul_mut(self, &other)
//     }

//     /* DIVISION */
//     /// Returns the result of dividing `other` from `self`
//     pub fn div(&self, other: &Self) -> Self {
//         Self {
//             x: self.x / other.x,
//             y: self.y / other.y,
//             z: self.z / other.z,
//             w: self.w / other.w
//         }
//     }

//     /// Divides `other` from self
//     pub fn div_mut(&mut self, other: &Self) -> &mut Self {
//         *self = self.div(other);
//         self
//     }

//     /// Divide all of the properties of `self` by a singular value of type `T`
//     pub fn div_by(&self, value: T) -> Self {
//         let other = Vec4::<T>::from(&[value]);
//         self.div(&other)
//     }

//     /// Divide all of the properties of `self` by a singular value of type `T`
//     pub fn div_by_mut(&mut self, value: T) -> &mut Self {
//         let other = Vec4::<T>::from(&[value]);
//         Self::div_mut(self, &other)
//     }

//     pub fn dot(&self, other: &Self) -> T {
//         self.x * other.x + self.y * other.y + self.z * other.z
//     }

//     /// Calculate the squared magnitude (length) of the vector
//     pub fn squared_magnitude(&self) -> T {
//         self.x * self.x + self.y * self.y + self.z * self.z
//     }

//     /// Calculate the magnitude (length) of the vector
//     pub fn magnitude(&self) -> T
//     where
//         T: Float,
//     {
//         self.squared_magnitude().sqrt()
//     }

//     /// Normalize the vector (make it a unit vector)
//     pub fn normalize(&self) -> Self
//     where
//         T: Float,
//     {
//         let mag = self.magnitude();
//         if mag != T::zero() {
//             return Self {
//                 x: self.x / mag,
//                 y: self.y / mag,
//                 z: self.z / mag,
//                 w: self.w / mag
//             }
//         }

//         *self
//     }

//     /// Mutably normalize the point. Ideally you'd call this function just once
//     pub fn normalize_mut(&mut self) -> &mut Self
//     where T: Float,
//     {
//         let mag = self.magnitude();
//         if mag != T::zero() {
//             self.div_by_mut(mag);
//         }

//         self
//     }

//     // Calculate the cross product between two vectors
//     pub fn cross(&self, other: &Self) -> Self {
//         Self {
//             x: self.y * other.z - self.z * other.y,
//             y: self.z * other.x - self.x * other.z,
//             z: self.x * other.y - self.y * other.x,
//             w: self.w * other.w - self.w * other.w,
//         }
//     }
// }