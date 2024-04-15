use crate::util::vec::*;

#[derive(Clone, Copy)]
pub struct Ray {
    pub position: Vec3,
    pub direction: Vec3
}

impl Ray {
    pub fn new(position: Vec3, direction: Vec3) -> Self {
        Self { position, direction }
    }

    pub fn get_point(&self, distance: f32) -> Vec2 {
        Vec2 {
            x: self.position.x + (self.direction.x * distance),
            y: self.position.y + (self.direction.y * distance)
        }
    }
}