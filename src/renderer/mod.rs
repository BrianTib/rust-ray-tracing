use std::borrow::Cow;
use wgpu::{hal::MAX_VERTEX_BUFFERS, util::DeviceExt};
use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::Window,
};

use crate::{
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

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 2],
    //pub color: [f32; 3],
}

impl Vertex {
    const ATTRIBS: [wgpu::VertexAttribute; 2] =
        wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3];

    fn desc() -> wgpu::VertexBufferLayout<'static> {
        use std::mem;

        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }

    fn is_equal(&self, other: &Vertex) -> bool {
        //self.position == other.position && self.color == other.color
        self.position == other.position
    }
}

pub struct Renderer {
    pub dimensions: (f32, f32),
    pub scene: Scene,
    pub vertex_buffer: Vec<Vertex>
}

impl Renderer {
    const MAX_VERTEX_BUFFER: usize = 2;

    pub fn new(dimensions: (f32, f32)) -> Self {
        let camera = Camera::new()
            .set_position(Vec3::new(0.0, 0.0, 5.0))
            .set_direction(Vec3::new(0.0, 0.0, -1.0));

        let scene = Scene::new()
            .add_camera(camera);

        let mut vertex_buffer = Vec::with_capacity(Self::MAX_VERTEX_BUFFER);
        let width = dimensions.0 as usize;
        let height = dimensions.1 as usize;
        //let aspect_ratio = dimensions.0 / dimensions.1;
        
        for y in 0..height {
            for x in 0..width {
                let x_normalized = (x as f32 / width as f32) * 2.0 - 1.0;
                let y_normalized = (y as f32 / height as f32) * 2.0 - 1.0;
                vertex_buffer.push(Vertex { position: [x_normalized, y_normalized] });
            }
        }

        Self {
            dimensions,
            scene,
            vertex_buffer
        }
    }

    pub fn setup_scene(&mut self) {
        let mut spheres: Vec<Sphere> = Vec::new();

        // Generate random spheres
        // let sphere_count = 100;

        // let position_range = -7.5..7.5;
        // let normal_range = 0.0..1.0;

        // for _ in 0..sphere_count {
        //     let sphere = Sphere::new(
        //         Vec3::new(random_range(position_range.clone()), random_range(position_range.clone()), random_range(position_range.clone())),
        //         random_range(normal_range.clone()),
        //         Material {
        //             albedo: Color::rgb(random_range(normal_range.clone()), random_range(normal_range.clone()), random_range(normal_range.clone())),
        //             roughness: random_range(normal_range.clone()),
        //             metallic: random_range(normal_range.clone())
        //         }
        //     );

        //     spheres.push(sphere);
        // }

        // Top sphere
        spheres.push(
            Sphere::new(
                Vec3::new(-1.7, 0.0, 0.0),
                1.0,
                Material {
                    albedo: Color::rgb(1.0, 0.0, 0.1),
                    roughness: 0.1,
                    metallic: 1.0,
                    ..Default::default()
                } 
            )
        );

        // Floor sphere
        spheres.push(
            Sphere::new(
                Vec3::new(0.1, 0.0, 0.0),
                1.0,
                Material {
                    albedo: Color::rgb(0.0, 0.5, 0.0),
                    roughness: 0.1,
                    //metallic: 1.0,
                    ..Default::default()
                } 
            )
        );

        let point_light = PointLight::new(
            Vec3::new(-0.5, -2.0, 0.0),
            Vec3::new(0.0, -1.0, -1.0),
            1.0
        );

        self.scene
            .add_shapes(spheres)
            .add_light(point_light);
    }

    pub fn start(&mut self) {
        env_logger::init();

        let event_loop = EventLoop::new().unwrap();
        let builder = winit::window::WindowBuilder::new()
            .with_title("Ray Tracer")
            .with_resizable(false)
            .with_inner_size(winit::dpi::LogicalSize::new(self.dimensions.0, self.dimensions.1));

        let window = builder.build(&event_loop).unwrap();

        pollster::block_on(self.run(event_loop, window));
    }

    async fn run(&mut self, event_loop: EventLoop<()>, window: Window) {
        let mut size = window.inner_size();
        size.width = size.width.max(1);
        size.height = size.height.max(1);
    
        let instance = wgpu::Instance::default();
    
        let surface = instance.create_surface(&window).unwrap();
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                force_fallback_adapter: false,
                // Request an adapter which can render to our surface
                compatible_surface: Some(&surface),
            })
            .await
            .expect("Failed to find an appropriate adapter");
    
        // Create the logical device and command queue
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    required_features: wgpu::Features::empty(),
                    // Make sure we use the texture resolution limits from the adapter, so we can support images the size of the swapchain.
                    required_limits: wgpu::Limits::downlevel_webgl2_defaults()
                        .using_resolution(adapter.limits()),
                },
                None,
            )
            .await
            .expect("Failed to create device");
    
        // Load the shaders from disk
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("ray_shader.wgsl"))),
        });
    
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });
    
        let swapchain_capabilities = surface.get_capabilities(&adapter);
        let swapchain_format = swapchain_capabilities.formats[0];
    
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[Vertex::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: swapchain_format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL
                })]
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::PointList,
                ..Default::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        });
    
        let mut config = surface
            .get_default_config(&adapter, size.width, size.height)
            .unwrap();
    
        surface.configure(&device, &config);

        //let vertices = self.scene.render(self.dimensions);

        //println!("{vertices:#?}");
        // let mut vertices = [
        //     Vertex { position: [0.0, 0.0, 0.0], color: [1.0, 0.0, 0.0 ] },
        //     // Vertex { position: [-0.4, 0.0, 0.0], color: [1.0, 0.0, 0.0 ] },
        //     // Vertex { position: [-0.4, -0.1, 0.0], color: [1.0, 0.0, 0.0 ] },
        // ];
    
        let window = &window;
        event_loop
            .run(move |event, target| {
                // Have the closure take ownership of the resources.
                // `event_loop.run` never returns, therefore we must do this to ensure
                // the resources are properly cleaned up.
                let _ = (&instance, &adapter, &shader, &pipeline_layout);
    
                if let Event::WindowEvent {
                    window_id: _,
                    event,
                } = event
                {
                    match event {
                        WindowEvent::Resized(new_size) => {
                            // Reconfigure the surface with the new size
                            config.width = new_size.width.max(1);
                            config.height = new_size.height.max(1);
                            self.dimensions = (config.width as f32, config.height as f32);
                            surface.configure(&device, &config);
                            // On macos the window needs to be redrawn manually after resizing
                            window.request_redraw();
                        }
                        WindowEvent::RedrawRequested => {
                            let frame = surface
                                .get_current_texture()
                                .expect("Failed to acquire next swap chain texture");
                            
                            let view = frame
                                .texture
                                .create_view(&wgpu::TextureViewDescriptor::default());

                            let vertex_buffer = device.create_buffer_init(
                                &wgpu::util::BufferInitDescriptor {
                                    label: None,
                                    contents: bytemuck::cast_slice(&self.vertex_buffer),
                                    usage: wgpu::BufferUsages::VERTEX,
                                }
                            );
                            
                            let mut encoder =
                                device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                                    label: None,
                                });

                            {
                                let mut rpass =
                                    encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                                        label: None,
                                        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                                            view: &view,
                                            resolve_target: None,
                                            ops: wgpu::Operations {
                                                load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                                                store: wgpu::StoreOp::Store,
                                            },
                                        })],
                                        depth_stencil_attachment: None,
                                        timestamp_writes: None,
                                        occlusion_query_set: None,
                                    });
                                    
                                rpass.set_pipeline(&render_pipeline);
                                rpass.set_vertex_buffer(0, vertex_buffer.slice(..));
                                rpass.draw(0..self.vertex_buffer.len() as u32, 0..1);
                            }
    
                            //println!("Redrawing");
                            queue.submit(Some(encoder.finish()));
                            frame.present();
                        }
                        WindowEvent::CloseRequested => target.exit(),
                        _ => {}
                    };
                }
            })
            .unwrap();
    }
}

/// Takes in an array of vertices and returns a vector of unique vertices and
/// a vector of the repeated indices
pub fn get_unique_vertices(vertices: &[Vertex]) -> (Vec<Vertex>, Vec<u16>) {
    let mut index_buffer: Vec<u16> = Vec::new();
    let mut unique_vertices: Vec<Vertex> = Vec::new();

    for vertex in vertices {
        // Check if the vertex is already in unique_vertices
        if let Some(index) = unique_vertices.iter().position(|v| v.is_equal(vertex)) {
            index_buffer.push(index as u16);
        } else {
            // If not, add it to unique_vertices
            let index = unique_vertices.len() as u16;
            unique_vertices.push(*vertex);
            index_buffer.push(index);
        }
    }

    (unique_vertices, index_buffer)
}

