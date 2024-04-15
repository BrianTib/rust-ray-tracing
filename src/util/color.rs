#![allow(dead_code)]
use crate::util::{Numeric, Float, vec::Vector, random_range};

#[derive(Debug, Clone, Copy, Hash, PartialEq)]
pub struct Color<T> {
    pub r: T,
    pub g: T,
    pub b: T,
    pub a: T
}

impl<T> Color<T>
where T: Numeric
{
    /// Create a new color using RGB values (range: 0.0 - 1.0)
    pub fn rgb(r: T, g: T, b: T) -> Self {
        Color { r, g, b, a: T::one() }
    }

    /// Create a new color using RGBA values (range: 0.0 - 1.0)
    pub fn rgba(r: T, g: T, b: T, a: T) -> Self {
        Color { r, g, b, a }
    }

    // /// Create a new color using HSL values (range: 0.0 - 1.0)
    // pub fn hsl(h: T, s: T, l: T) -> Self {
    //     let (r, g, b) = Self::_hsl_to_rgb(h, s, l);
    //     Color { r, g, b, a: T::one() }
    // }

    // /// Create a new color using HSLa values (range: 0.0 - 1.0)
    // pub fn hsla(h: T, s: T, l: T, a: T) -> Self {
    //     let (r, g, b) = Self::_hsl_to_rgb(h, s, l);
    //     Color { r, g, b, a }
    // }

    // fn _hsl_to_rgb(h: T, s: T, l: T) -> (T, T, T) {
    //     let h = h.into();

    //     // Convert HSL to RGB
    //     let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    //     let x = c * (1.0 - ((h * 6.0) % 2.0 - 1.0).abs());
    //     let m = l - c / 2.0;

    //     let (r, g, b) = if h < 1.0 / 6.0 {
    //         (c, x, 0.0)
    //     } else if h < 2.0 / 6.0 {
    //         (x, c, 0.0)
    //     } else if h < 3.0 / 6.0 {
    //         (0.0, c, x)
    //     } else if h < 4.0 / 6.0 {
    //         (0.0, x, c)
    //     } else if h < 5.0 / 6.0 {
    //         (x, 0.0, c)
    //     } else {
    //         (c, 0.0, x)
    //     };

    //     (r + m, g + m, b + m)
    // }
}

impl<T> Vector<T> for Color<T>
where T: Numeric
{
    fn from(v: &[T]) -> Self {
        match v.len() {
            1 => Self { r: v[0], g: v[0], b: v[0], a: v[0] },
            2 => Self { r: v[0], g: v[1], b: T::zero(), a: T::zero() },
            3 => Self { r: v[0], g: v[1], b: v[2], a: T::zero() },
            4 => Self { r: v[0], g: v[1], b: v[2], a: v[3] },
            _ => Self { r: T::zero(), g: T::zero(), b: T::zero(), a: T::zero() }
        }
    }

    fn copy(other: &Self) -> Self {
        Self {
            r: other.r,
            g: other.g,
            b: other.b,
            a: other.a
        }
    }

    fn random(min: T, max: T) -> Self
    where T: Numeric
    {
        Self {
            r: random_range(min..max),
            g: random_range(min..max),
            b: random_range(min..max),
            a: random_range(min..max)
        }
    }

    /* OPERATIONS */
    fn add(&self, other: &Self) -> Self {
        Self {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
            a: self.a + other.a
        }
    }

    fn sub(&self, other: &Self) -> Self {
        Self {
            r: self.r - other.r,
            g: self.g - other.g,
            b: self.b - other.b,
            a: self.a - other.a
        }
    }

    fn mul(&self, other: &Self) -> Self {
        Self {
            r: self.r * other.r,
            g: self.g * other.g,
            b: self.b * other.b,
            a: self.a * other.a
        }
    }

    fn div(&self, other: &Self) -> Self {
        Self {
            r: self.r / other.r,
            g: self.g / other.g,
            b: self.b / other.b,
            a: self.a / other.a
        }
    }

    /* VECTOR FUNCTIONS */
    fn cross(&self, other: &Self) -> Self {
        Self {
            r: self.g * other.b - self.b * other.g,
            g: self.b * other.r - self.r * other.b,
            b: self.r * other.g - self.g * other.r,
            a: self.a * other.a
        }
    }

    fn dot(&self, other: &Self) -> T where T: Float {
        self.r * other.r + self.g * other.g + self.b * other.b + self.a * other.a
    }

    fn squared_magnitude(&self) -> T where T: Float {
        self.r * self.r + self.g * self.g + self.b * self.b + self.a * self.a
    }
}