use crate::util::vec::*;

#[derive(Clone)]
pub struct Camera {
    pub position: Vec3<f32>,
    pub direction: Vec3<f32>,
    pub rays: Vec<Vec3<f32>>,
    pub fov: f32,
    pub near_clip: f32,
    pub far_clip: f32
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            position: Vec3::new(0.0, 0.0, 5.0),
            direction: Vec3::new(0.0, 0.0, -1.0),
            rays: Vec::new(),
            fov: 45.0,
            near_clip: 1.0,
            far_clip: 100.0
        }
    }
}

impl Camera {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_ray_directions(&mut self, width: u32, height: u32, aspect_ratio: f32) -> &Vec<Vec3<f32>> {
        self.rays = Vec::new();

        for y in 0..height {
            for x in 0..width {
                let x = (x as f32 / width as f32) / aspect_ratio;
                let y = y as f32 / height as f32;
                
                let coord = Vec2::new(x, y)
                    .mul_by(2.0)
                    .sub_by(1.0);

                let ray_direction = Vec3::new(coord.x, coord.y, -self.direction.z).normalize();
                self.rays.push(ray_direction);
            }
        }

        &self.rays
    }

    pub fn set_position(mut self, position: Vec3<f32>) -> Self {
        self.position = position;
        self
    }

    pub fn set_direction(mut self, direction: Vec3<f32>) -> Self {
        self.direction = direction.mul_by(-1.0);
        self
    }
}