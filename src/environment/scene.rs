use image::{ImageBuffer, Rgba};
//use num_traits::Pow;
use std::{time::Instant, io::Write};
//use std::io;

use super::{
    camera::Camera,
    light::Light
};

use crate::{
    shapes::Shape,
    util::{Color, Ray, vec::*},
};

pub type RenderSpace = u8;
pub type RenderFormat = Rgba<RenderSpace>;
pub type RendarableImage = ImageBuffer<RenderFormat, Vec<RenderSpace>>;

/// A scene with technically infinite dimensions
pub struct Scene {
    pub image: RendarableImage,
    pub cameras: Vec<Camera>,
    pub shapes: Vec<Box<dyn Shape>>,
    pub lights: Vec<Box<dyn Light>>
}

impl Default for Scene {
    fn default() -> Self {
        Self {
            image: ImageBuffer::new(200, 200),
            lights: Vec::new(),
            shapes: Vec::new(),
            cameras: Vec::new()
        }
    }
}

impl Scene {
    /// Creates a new scene
    /// `width` and `heigh` refer to the size of the images being output by the renderer
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            image: ImageBuffer::new(width, height),
            ..Default::default()
        }
    }

    /// Adds a [Shape] object to the scene
    pub fn add_shape<T>(&mut self, shape: T) -> &mut Self where T: Shape {
        Self::add_shapes(self, vec![shape])
    }

    /// Adds multiple [Shape] objects to the scene
    pub fn add_shapes<T>(&mut self, shapes: Vec<T>) -> &mut Self where T: Shape {
        for shape in shapes {
            self.shapes.push(Box::new(shape));
        }
        self
    }

    /// Adds a [Light] object to the scene
    pub fn add_light<T>(&mut self, light: T) -> &mut Self
    where
        T: Light
    {
        Self::add_lights(self, vec![light])
    }

    /// Adds multiple [Light] objects to the scene
    pub fn add_lights<T>(&mut self, lights: Vec<T>) -> &mut Self
    where
        T: Light
    {
        for light in lights {
            self.lights.push(Box::new(light));
        }
        self
    }

    /// Adds a camera to the array of cameras
    /// when [`Scene::render()`] is called, resulting images
    /// are made from all of the given cameras
    pub fn add_camera(&mut self, camera: Camera) -> &mut Self {
        self.cameras.push(camera);
        self
    }

    /// Output all of the views from all of the `.cameras` as .png images
    pub fn render(&mut self) {
        if self.cameras.len() <= 0 {
            panic!("No cameras to render!");
        }
        
        for i in 0..self.cameras.len() {
            self.render_camera(i);
        }
    }

    /// Render the view from a camera at the given index
    pub fn render_camera(&mut self, camera_index: usize) {
        let (width, height) = self.image.dimensions();
        let aspect_ratio = width as f32 / height as f32;

        let time = Instant::now();
        let mut colors: Vec<RenderFormat> = Vec::with_capacity((width * height) as usize);

        let camera_position = self.cameras[camera_index].position.clone();
        let ray_directions = self.cameras[camera_index].get_ray_directions(width, height, aspect_ratio);

        for y in 0..height {
            //print!("Rendering image... Rows left: {}", height - y);
            //io::stdout().flush().unwrap();
            
            for x in 0..width {
                let index = (x + y * width) as usize;

                let mut ray = Ray::new(
                    camera_position,
                    ray_directions[index]
                );

                let color = evaluate_pixel(
                    &mut ray,
                    &self.shapes,
                    &self.lights
                );

                colors.push(RenderFormat::from([
                    (color.r * 255.0) as RenderSpace,
                    (color.g * 255.0) as RenderSpace,
                    (color.b * 255.0) as RenderSpace,
                    (color.a * 255.0) as RenderSpace
                ]));        
            }

            //std::thread::sleep(std::time::Duration::from_millis(2));
            //print!("\x1b[2K\r");
        }

        let time = Instant::now() - time;
        println!("Rendering image #{} took: ~{:?}. Saving image...", camera_index + 1, time);

        for y in 0..height {
            for x in 0..width {
                self.image.put_pixel(x, y, colors[(x + y * width) as usize]);
            }

            // if y % 100 == 0 {
            //     // Save the image to a file
            //     self.image.save("output.png").unwrap();
            // }
        }

        // Save the image to a file
        self.image.save(format!("camera{}.png", camera_index + 1)).unwrap();
    }
}

fn evaluate_pixel(
    ray: &mut Ray,
    shapes: &[Box<dyn Shape>],
    lights: &[Box<dyn Light>]
) -> Color<f32> {
    let sky_color = Color::rgb(0.2, 0.2, 0.2);

    let mut color = Color::rgb(0.0, 0.0, 0.0);
    let mut multiplier = 1.0;

    for _ in 0..10 {
        let shape_hit = shoot_ray(&ray, &shapes);
        if shape_hit.is_none() {
            color.add_mut(&sky_color.mul_by(multiplier));
            break;
        }
        
        // At this point, we have the closest object the ray hit
        let (shape_index, hit_distance) = shape_hit.unwrap();
        let shape = &shapes[shape_index];
        let material = shape.get_material();

        let (hit_color, hit_position, hit_normal) = get_hit_color(
            &ray, 
            &shapes[shape_index], 
            hit_distance, 
            lights
        );

        // Add the color of this ray bounce
        // to the total color to be displayed by the pixel
        color.add_mut(&hit_color.mul_by(multiplier));
        multiplier *= 0.5;

        // Move the ray backwards a tiny bit
        // So that we dont collide with ourselves
        ray.position = hit_normal.mul_by(0.0001).add(&hit_position);

        // Shift the direction a little bit based on the roughness of the material
        // and some randomness
        let roughness = material.roughness * Vec3::random_value(-0.5, 0.5);

        ray.direction = ray.direction.reflect(&hit_normal.add_by(roughness));
    }

    color
}

fn shoot_ray(ray: &Ray, shapes: &[Box<dyn Shape>]) -> Option<(usize, f32)> {
    if shapes.is_empty() { return None; }

    let mut closest_hit_distance = f32::MAX;
    let mut closest_shape_index = 0;

    for (i, shape) in shapes.iter().enumerate() {
        if !shape.is_3d() || !shape.has_radius() { continue; }

        let shape_position = shape.get_position();
        let shape_radius = shape.get_radius().unwrap();

        let origin = ray.position.sub(&shape_position);

        let a = ray.direction.dot(&ray.direction);
        let b = 2.0 * origin.dot(&ray.direction);
        let c = origin.dot(&origin) - shape_radius.powi(2);

        // Determine the amount of intersactions between the ray
        // and the radius of the circle
        // < 0 means there is no collision
        // = 0 means there is one collision
        // > 0 means there are two collisions
        // Quadratic formula discriminant:
        // b^2 -4ac
        let discriminant = b * b - 4.0 * a * c;

        if discriminant >= 0.0 {
            let closest_hit = (-b - discriminant.sqrt()) / (2.0 * a);
            if closest_hit > 0.0 && closest_hit < closest_hit_distance {
                closest_hit_distance = closest_hit;
                closest_shape_index = i;
            }
        }
    }

    // This means there was no hit, stop here
    if closest_hit_distance == f32::MAX { return None; }

    Some((closest_shape_index, closest_hit_distance))
}

fn get_hit_color(
    ray: &Ray,
    shape: &Box<dyn Shape>,
    hit_distance: f32,
    lights: &[Box<dyn Light>]
) -> (Color<f32>, Vec3<f32>, Vec3<f32>) {
    let shape_position = shape.get_position();
    let shape_color = shape.get_surface_color();

    let origin = ray.position.sub(&shape_position);
    let hit_position = ray.direction
        .mul_by(hit_distance)
        .add(&origin);

    let normal = hit_position.normalize();
    let light_intensity = lights[0].get_intensity(&normal);

    (shape_color.mul_by(light_intensity), hit_position, normal)
}