use std::{ops::Range, time::SystemTime};

pub mod vec;
pub mod matrix;

mod ray;
pub use ray::Ray;

mod color;
pub use color::Color;

mod material;
pub use material::Material;

// In-house function for generating simple randomness without importing an entire crate
/// Generates a random f32 between 0.0 - 1.0
pub fn random() -> f32 {
    // Create a seed from UNIX EPOCH until now in nanoseconds
    let seed = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("Time went backwards")
        .as_nanos()
        // Reversing the bits of the seed can help improve randomness.
        // This is particularly useful when the seed is derived from time,
        // as it helps to disperse the bits and avoid patterns related to
        // the system's clock.
        .reverse_bits();

    // Reduce the size of the seed to avoid overflow
    let seed = seed / 10e+28_f64 as u128;
    // Divide by the closest maximum value depending on the amount of digits
    let max = 10u128.pow(seed.to_string().len() as u32) - 1;

    seed as f32 / max as f32
}

pub fn random_lcg() -> f32 {
    // Constants for the linear congruential generator
    let m: u64 = 2_u64.pow(32);
    let a: u64 = 1664525;
    let c: u64 = 1013904223;

    // Linear Congruential Generator
    let lcg = |x: &mut u64, a: u64, c: u64, m: u64| -> u64 {
        *x = (a * (*x) + c) % m;
        *x
    };

    // Initial state (seed)
    let mut x: u64 = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();
    
    // Generate a random u64 and map it to a f32 between 0 and 1
    let random_u64 = lcg(&mut x, a, c, m);
    random_u64 as f32 / m as f32
}

pub fn random_range(range: Range<f32>) -> f32 {
    random() * (range.end - range.start + 1.0) + range.start
}