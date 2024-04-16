use super::Shape;
use crate::{
    util::{
        Material,
        Color,
        vec::*
    },
    renderer::Vertex
};

#[derive(Debug)]
pub struct Sphere {
    pub radius: f32,
    pub position: Vec3,
    pub material: Material
}

impl Sphere {
    pub fn new(position: Vec3, radius: f32, material: Material) -> Self {
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
    fn get_surface_color(&self) -> Color { self.material.albedo }
    fn get_position(&self) -> Vec3 { self.position }
    fn get_radius(&self) -> Option<f32> { Some(self.radius) }
    fn get_material(&self) -> Material { self.material }

    fn get_vertices(&self) -> &[Vertex] {
        &[]
    }
}