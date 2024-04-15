use ray_tracing::{
    shapes::Sphere,
    environment::{
        scene::Scene,
        camera::Camera,
        light::PointLight
    },
    util::{
        Color,
        Material,
        vec::*,
        random_range
    }
};

fn main() {
    let (width, height) = (720, 720);

    let camera_one = Camera::new()
        .set_position(Vec3::new(0.0, 0.0, 5.0))
        .set_direction(Vec3::new(0.0, 0.0, -1.0));

    // let camera_two = Camera::new()
    //     .set_position(Vec3::new(0.0, 0.0, -5.0))
    //     .set_direction(Vec3::new(0.0, 0.0, 1.0));

    let mut spheres: Vec<f32> = Vec::new();

    // Generate random spheres
    let sphere_count = 100;

    let position_range = random_range(-7.5..7.5);
    let normal_range = random_range(0.0..1.0);

    // for _ in 0..sphere_count {
    //     let sphere = Sphere::new(
    //         Vec3::new(position_range.sample(&mut rng), position_range.sample(&mut rng), position_range.sample(&mut rng)),
    //         normal_range.sample(&mut rng),
    //         Material {
    //             albedo: Color::rgb(normal_range.sample(&mut rng), normal_range.sample(&mut rng), normal_range.sample(&mut rng)),
    //             roughness: normal_range.sample(&mut rng),
    //             metallic: normal_range.sample(&mut rng)
    //         }
    //     );

    //     spheres.push(sphere);
    // }

    // Top sphere
    // spheres.push(
    //     Sphere::new(
    //         Vec3::new(1.7, 0.0, 0.0),
    //         1.0,
    //         Material {
    //             albedo: Color::rgb(0.0, 1.0, 1.0),
    //             roughness: 0.1,
    //             //metallic: 1.0,
    //             ..Default::default()
    //         } 
    //     )
    // );

    // // Floor sphere
    // spheres.push(
    //     Sphere::new(
    //         Vec3::new(-1.7, 0.0, 0.0),
    //         1.0,
    //         Material {
    //             albedo: Color::rgb(0.0, 0.5, 1.0),
    //             roughness: 0.04,
    //             //metallic: 1.0,
    //             ..Default::default()
    //         } 
    //     )
    // );

    let point_light = PointLight::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, -1.0),
        1.0
    );

    let mut scene = Scene::new(width, height);
    scene
        .add_light(point_light)
        //.add_shapes(spheres)
        .add_camera(camera_one)
        //.add_camera(camera_two)
        .render();
}