use crate::util::random_range;

mod vec2;
pub use vec2::Vec2;

mod vec3;
pub use vec3::Vec3;

mod vec4;
pub use vec4::Vec4;

/// Determine global functionality for Vectors regardless of their dimensions
pub trait Vector: Copy + Sized {
    fn from(v: &[f32]) -> Self;
    fn copy(other: &Self) -> Self;
    fn random(min: f32, max: f32) -> Self;
    
    /* ADDITION */
    /// Returns the result of adding `other` to `self`
    fn add(&self, other: &Self) -> Self;
    
    /// Mutably adds `other` to `self`
    fn add_mut(&mut self, other: &Self) -> &mut Self {
        *self = self.add(other);
        self
    }
    
    /// Add value of type `T` to all of the properties of `self`
    fn add_by(&self, value: f32) -> Self {
        let other = Self::from(&[value]);
        self.add(&other)
    }

    /// Mutably add value of type `T` to all of the properties of `self`
    fn add_by_mut(&mut self, value: f32) -> &mut Self {
        let other = Self::from(&[value]);
        Self::add_mut(self, &other)
    }

    /* SUBSTRACTION */
    /// Returns the result of substracting `other` from `self`
    fn sub(&self, other: &Self) -> Self;
    
    /// Mutably substracts `other` from `self`
    fn sub_mut(&mut self, other: &Self) -> &mut Self {
        *self = self.sub(other);
        self
    }
    
    /// Substract `value` of type `[T]` from all of the properties of `self`
    fn sub_by(&self, value: f32) -> Self {
        let other = Self::from(&[value]);
        self.sub(&other)
    }

    /// Mutably substract `value` of type `T` to all of the properties of `self`
    fn sub_by_mut(&mut self, value: f32) -> &mut Self {
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
    fn mul_by(&self, value: f32) -> Self {
        let other = Self::from(&[value]);
        self.mul(&other)
    }

    /// Mutably multiplies `value` of type `T` with all of the properties of `self`
    fn mul_by_mut(&mut self, value: f32) -> &mut Self {
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
    fn div_by(&self, value: f32) -> Self {
        let other = Self::from(&[value]);
        self.div(&other)
    }

    /// Mutably divide all of the properties of `self` by `value` of type `T`
    fn div_by_mut(&mut self, value: f32) -> &mut Self {
        let other = Self::from(&[value]);
        Self::div_mut(self, &other)
    }

    /* VECTOR OPERATIONS */
    /// Calculate the cross product between two vectors
    fn cross(&self, other: &Self) -> Self;
    fn dot(&self, other: &Self) -> f32;

    /// Calculate the squared magnitude (length) of the vector
    fn squared_magnitude(&self) -> f32;

    /// Calculate the magnitude (length) of the vector
    fn magnitude(&self) -> f32 {
        self.squared_magnitude().sqrt()
    }

    // Return the result of inverting the vector
    fn invert(&self) -> Self {
        self.mul_by(-1.0)
    }

    /// Mutably invert the vector
    fn invert_mut(&mut self) -> &mut Self {
        *self = self.invert();
        self
    }

    /// Normalize the vector (make it a unit vector)
    fn normalize(&self) -> Self {
        let mag = self.magnitude();
        if mag != 0.0 { return self.div_by(mag); }
        *self
    }

    fn normalize_mut(&mut self) -> &mut Self {
        *self = self.normalize();
        self
    }

    fn reflect(&mut self, normal: &Self) -> Self {
        let dot = self.dot(&normal);
        let normal_scaled = normal.mul_by(dot * 2.0);
        self.sub(&normal_scaled)
    }

    fn random_value(min: f32, max: f32) -> f32 
    {
        random_range(min..max)
    }
}