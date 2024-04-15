use crate::util::{Numeric, Float, random_range};

mod vec2;
pub use vec2::Vec2;

mod vec3;
pub use vec3::Vec3;

mod vec4;
pub use vec4::Vec4;

/// Determine global functionality for Vectors regardless of their dimensions
pub trait Vector<T>: Copy + Sized {
    fn from(v: &[T]) -> Self where T: Numeric;
    fn copy(other: &Self) -> Self;
    fn random(min: T, max: T) -> Self where T: Numeric;
    
    /* ADDITION */
    /// Returns the result of adding `other` to `self`
    fn add(&self, other: &Self) -> Self;
    
    /// Mutably adds `other` to `self`
    fn add_mut(&mut self, other: &Self) -> &mut Self {
        *self = self.add(other);
        self
    }
    
    /// Add value of type `T` to all of the properties of `self`
    fn add_by(&self, value: T) -> Self where T: Numeric {
        let other = Self::from(&[value]);
        self.add(&other)
    }

    /// Mutably add value of type `T` to all of the properties of `self`
    fn add_by_mut(&mut self, value: T) -> &mut Self where T: Numeric {
        let other = Self::from(&[value]);
        Self::add_mut(self, &other)
    }

    /* SUBSTRACTION */
    /// Returns the result of substracting `other` from `self`
    fn sub(&self, other: &Self) -> Self;
    
    /// Mutably substracts `other` from `self`
    fn sub_mut(&mut self, other: &Self) -> &mut Self where T: Numeric {
        *self = self.sub(other);
        self
    }
    
    /// Substract `value` of type `[T]` from all of the properties of `self`
    fn sub_by(&self, value: T) -> Self where T: Numeric {
        let other = Self::from(&[value]);
        self.sub(&other)
    }

    /// Mutably substract `value` of type `T` to all of the properties of `self`
    fn sub_by_mut(&mut self, value: T) -> &mut Self where T: Numeric {
        let other = Self::from(&[value]);
        Self::sub_mut(self, &other)
    }

    /* MULTIPLICATION */
    /// Returns the result of multiplying `other` with `self`
    fn mul(&self, other: &Self) -> Self;
    
    /// Mutably multiply `other` with `self`
    fn mul_mut(&mut self, other: &Self) -> &mut Self {
        *self = self.mul(other);
        self
    }
    
    /// Multiplies `value` of type `[T]` with all of the properties of `self`
    fn mul_by(&self, value: T) -> Self where T: Numeric {
        let other = Self::from(&[value]);
        self.mul(&other)
    }

    /// Mutably multiplies `value` of type `T` with all of the properties of `self`
    fn mul_by_mut(&mut self, value: T) -> &mut Self where T: Numeric {
        let other = Self::from(&[value]);
        Self::mul_mut(self, &other)
    }

    /* DIVISION */
    /// Returns the result of dividing `self` by `other`
    fn div(&self, other: &Self) -> Self;
    
    /// Mutably divide `self` by `other`
    fn div_mut(&mut self, other: &Self) -> &mut Self {
        *self = self.div(other);
        self
    }
    
    /// Divide all of the properties of `self` by `value` of type `[T]`
    fn div_by(&self, value: T) -> Self where T: Numeric {
        let other = Self::from(&[value]);
        self.div(&other)
    }

    /// Mutably divide all of the properties of `self` by `value` of type `T`
    fn div_by_mut(&mut self, value: T) -> &mut Self where T: Numeric {
        let other = Self::from(&[value]);
        Self::div_mut(self, &other)
    }

    /* VECTOR OPERATIONS */
    /// Calculate the cross product between two vectors
    fn cross(&self, other: &Self) -> Self;
    fn dot(&self, other: &Self) -> T where T: Numeric;

    /// Calculate the squared magnitude (length) of the vector
    fn squared_magnitude(&self) -> T where T: Numeric;

    /// Calculate the magnitude (length) of the vector
    fn magnitude(&self) -> T where T: Numeric {
        self.squared_magnitude().sqrt()
    }

    // Return the result of inverting the vector
    fn invert(&self) -> Self where T: Numeric {
        self.mul_by(-T::one())
    }

    /// Mutably invert the vector
    fn invert_mut(&mut self) -> &mut Self where T: Numeric {
        *self = self.invert();
        self
    }

    /// Normalize the vector (make it a unit vector)
    fn normalize(&self) -> Self where T: Numeric {
        let mag = self.magnitude();
        if mag != T::zero() { return self.div_by(mag); }
        *self
    }

    fn normalize_mut(&mut self) -> &mut Self where T: Numeric {
        *self = self.normalize();
        self
    }

    fn reflect(&mut self, normal: &Self) -> Self where T: Numeric {
        let dot = self.dot(&normal);
        let normal_scaled = normal.mul_by(dot * T::from(2).unwrap());
        self.sub(&normal_scaled)
    }

    fn random_value(min: T, max: T) -> T 
    where T: Numeric {
        random_range(min..max)
    }
}