use super::Shape;
use crate::util::{Material, Color, vec::*};
pub struct Circle {
    pub position: Vec3,
    pub radius: f32,
    pub material: Material
}

impl Shape for Circle {
    fn has_radius(&self) -> bool { true }
    fn is_3d(&self) -> bool { false }
    
    fn get_surface_color(&self) -> Color { self.material.albedo }
    fn get_position(&self) -> Vec3 { self.position }
    fn get_radius(&self) -> Option<f32> { Some(self.radius) }
    fn get_material(&self) -> Material { self.material }
}