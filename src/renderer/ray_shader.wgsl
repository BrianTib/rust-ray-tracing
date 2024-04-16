// Every pixel in the screen
struct PixelInput {
    @location(0) coord: vec2<f32>
};

struct PixelOutput {
    @builtin(position) coord: vec4<f32>,
    @location(0) color: vec4<f32>
};

// struct Ray {
//     @builtin(position) origin: vec3<f32>,
//     @location(0) direction: vec3<f32>
// }

@vertex
fn vs_main(pixel: PixelInput) -> PixelOutput {
    var output: PixelOutput;
    output.coord = vec4<f32>(pixel.coord, 1.0, 1.0);

    let r = (output.coord.x + 1.0) / 2.0;
    let g = (output.coord.y + 1.0) / 2.0;
    
    let ray_origin = vec3<f32>(0.0, 0.0, 2.0);
    let ray_direction = vec3<f32>(output.coord.x, output.coord.y, -1.0);
    let radius = 0.5;

    let a = dot(ray_direction, ray_direction);
    let b = dot(ray_origin, ray_direction) * 2.0;
    let c = dot(ray_origin, ray_origin) - pow(radius, 2.0);

    let discriminant = pow(b, 2.0) - 4.0 * a * c;

    if (discriminant >= 0.0) {
        output.color = vec4<f32>(1.0, 0.0, 1.0, 1.0);
    } else {
        //output.color = vec4<f32>(r, g, 0.0, 1.0);
        output.color = vec4<f32>(0.0, 0.0, 0.0, 1.0);
    }

    //output.color = vec4<f32>(r, g, 0.0, 1.0);
    return output;
}

// Fragment shader
@fragment
fn fs_main(in: PixelOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color);
}