use super::Shape;
use crate::util::{Material, Color, vec::*};

#[derive(Debug)]
pub struct Sphere {
    pub radius: f32,
    pub position: Vec3<f32>,
    pub material: Material
}

impl Sphere {
    pub fn new(position: Vec3<f32>, radius: f32, material: Material) -> Self {
        Self {
            radius,
            position,
            material
        }
    }
}

impl Shape for Sphere {
    fn has_radius(&self) -> bool { true }
    fn is_3d(&self) -> bool { true }
    fn get_surface_color(&self) -> Color<f32> { self.material.albedo }
    fn get_position(&self) -> Vec3<f32> { self.position }
    fn get_radius(&self) -> Option<f32> { Some(self.radius) }
    fn get_material(&self) -> Material { self.material }
}