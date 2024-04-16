use image::Rgba;
//use num_traits::Pow;
use std::time::Instant;
//use std::io;

use super::{
    camera::Camera,
    light::Light
};

use crate::{
    shapes::Shape,
    util::{Color, Ray, vec::*},
    renderer::Vertex
};

pub type RenderSpace = u8;
pub type RenderFormat = Rgba<RenderSpace>;

/// A scene with technically infinite dimensions
pub struct Scene {
    pub camera: Camera,
    pub shapes: Vec<Box<dyn Shape>>,
    pub lights: Vec<Box<dyn Light>>
}

impl Default for Scene {
    fn default() -> Self {
        Self {
            lights: Vec::new(),
            shapes: Vec::new(),
            camera: Camera::new()
        }
    }
}

impl Scene {
    /// Creates a new scene
    /// `width` and `heigh` refer to the size of the images being output by the renderer
    pub fn new() -> Self {
        Self::default()
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
    pub fn add_camera(mut self, camera: Camera) -> Self {
        self.camera = camera;
        self
    }

    /// Output all of the views from all of the `.cameras` as .png images
    pub fn render(&mut self, dimensions: (f32, f32)) -> Vec<Vertex> {        
        self.render_camera(dimensions)
    }

    /// Render the view from a camera at the given index
    pub fn render_camera(&mut self, dimensions: (f32, f32)) -> Vec<Vertex>{
        let (width, height) = dimensions;
        //let aspect_ratio = width as f32 / height as f32;
        let mut vertices: Vec<Vertex> = Vec::with_capacity((width * height) as usize);

        //let camera_position = self.camera.position.clone();
        //let ray_directions = self.camera.get_ray_directions(width, height, aspect_ratio);

        for y in 0..height as u32 {
            //print!("Rendering image... Rows left: {}", height - y);
            //io::stdout().flush().unwrap();
            
            for x in 0..width as u32 {
                // let index = (x + y * width) as usize;

                // let mut ray = Ray::new(
                //     camera_position,
                //     ray_directions[index]
                // );

                // let color = evaluate_pixel(
                //     &mut ray,
                //     &self.shapes,
                //     &self.lights
                // );

                let x = ((x as f32 / width as f32) * 2.0) - 1.0;
                let y = ((y as f32 / height as f32) * 2.0) - 1.0;

                vertices.push(Vertex {
                    position: [x, y],
                    //color: [1.0, 1.0, 1.0]
                }); 

                // vertices.push(Vertex {
                //     position: [x as f32, y as f32, 0.0],
                //     color: [color.r, color.g, color.b]
                // });       
            }
        }

        return vertices;
    }
}

fn evaluate_pixel(
    ray: &mut Ray,
    shapes: &[Box<dyn Shape>],
    lights: &[Box<dyn Light>]
) -> Color {
    let sky_color = Color::rgb(0.005, 0.005, 0.005);

    let mut color = Color::rgb(0.0, 0.0, 0.0);
    let mut multiplier = 1.0;

    for _ in 0..10 {
        let shape_hit = shoot_ray(&ray, &shapes);
        //println!("{:?}", shape_hit);
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
) -> (Color, Vec3, Vec3) {
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