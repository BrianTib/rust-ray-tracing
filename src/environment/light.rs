use crate::util::vec::*;

pub trait Light: 'static {
    fn get_intensity(&self, normal: &Vec3) -> f32;
}

#[derive(Clone, Copy)]
pub struct PointLight {
    pub position: Vec3,
    pub direction: Vec3,
    pub intensity: f32
}

impl Light for PointLight {
    fn get_intensity(&self, normal: &Vec3) -> f32 {
        normal.dot(&self.direction).max(0.0) * self.intensity
    }
}

impl PointLight {
    pub fn new(position: Vec3, direction: Vec3, intensity: f32) -> Self {
        Self {
            position,
            direction: direction.mul_by(-1.0).normalize(),
            intensity
        }
    }
}
