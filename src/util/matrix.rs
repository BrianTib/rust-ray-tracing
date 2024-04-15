#![allow(dead_code)]
use crate::util::Numeric;

pub struct Matrix4<T> {
    pub data: [[T; 4]; 4]
}

pub struct Matrix3<T> {
    pub data: [[T; 3]; 3]
}

impl<T> Matrix3<T>
where T: Numeric {
    pub fn new(data: [[T; 3]; 3]) -> Self where T: Numeric {
        Self { data }
    }
}

impl<T> Matrix4<T>
where T: Numeric {
    pub fn new(data: [[T; 4]; 4]) -> Self {
        Self { data }
    }
}