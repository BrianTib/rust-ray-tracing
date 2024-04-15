#![allow(dead_code)]
pub struct Matrix4 {
    pub data: [[f32; 4]; 4]
}

pub struct Matrix3 {
    pub data: [[f32; 3]; 3]
}

impl Matrix3 {
    pub fn new(data: [[f32; 3]; 3]) -> Self {
        Self { data }
    }
}

impl Matrix4 {
    pub fn new(data: [[f32; 4]; 4]) -> Self {
        Self { data }
    }
}