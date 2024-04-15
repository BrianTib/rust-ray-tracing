use crate::util::Color;

#[derive(Debug, Clone, Copy)]
pub struct Material {
    pub albedo: Color,
    pub roughness: f32,
    pub metallic: f32
}

impl Default for Material {
    fn default() -> Self {
        Self {
            albedo: Color::rgb(0.0, 0.0, 0.0),
            roughness: 1.0,
            metallic: 0.0
        }
    }
}

impl Material {
    pub fn new(albedo: Color, roughness: f32, metallic: f32) -> Self {
        Self { albedo, roughness, metallic }
    }
}