mod circle;
mod sphere;

pub use circle::Circle;
pub use sphere::Sphere;

use crate::util::{Color, Material, vec::Vec3};

pub trait Shape: 'static  {
    fn has_radius(&self) -> bool;
    fn is_3d(&self) -> bool;
    fn get_radius(&self) -> Option<f32>;
    fn get_position(&self) -> Vec3;
    fn get_surface_color(&self) -> Color;
    fn get_material(&self) -> Material;
}