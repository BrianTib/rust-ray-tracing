use std::borrow::Cow;
use wgpu::util::DeviceExt;
use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::Window,
};

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
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
        self.position == other.position && self.color == other.color
    }
}

pub struct Renderer {}

impl Renderer {
    pub fn start() {
        env_logger::init();

        let event_loop = EventLoop::new().unwrap();
        let builder = winit::window::WindowBuilder::new()
            .with_title("Ray Tracer")
            .with_inner_size(winit::dpi::LogicalSize::new(1280.0, 720.0));

        let window = builder.build(&event_loop).unwrap();

        pollster::block_on(Self::run(event_loop, window));
    }

    async fn run(event_loop: EventLoop<()>, window: Window) {
        let mut size = window.inner_size();
        size.width = size.width.max(1);
        size.height = size.height.max(1);
    
        let instance = wgpu::Instance::default();
    
        const VERTICES: &[Vertex] = &[
            Vertex { position: [-0.0868241, 0.49240386, 0.0], color: [0.5, 0.0, 0.5] }, // A
            Vertex { position: [-0.49513406, 0.06958647, 0.0], color: [0.5, 0.0, 0.5] }, // B
            Vertex { position: [-0.21918549, -0.44939706, 0.0], color: [0.5, 0.0, 0.5] }, // C
            Vertex { position: [0.35966998, -0.3473291, 0.0], color: [0.5, 0.0, 0.5] }, // D
            Vertex { position: [0.44147372, 0.2347359, 0.0], color: [0.5, 0.0, 0.5] }, // E
        ];
    
        const INDICES: &[u16] = &[
            0, 1, 4,
            1, 2, 4,
            2, 3, 4,
        ];
    
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
    
        let vertex_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(VERTICES),
                usage: wgpu::BufferUsages::VERTEX,
            }
        );
    
        let index_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(INDICES),
                usage: wgpu::BufferUsages::INDEX,
            }
        );
    
        let num_indices = INDICES.len() as u32;
    
        // Load the shaders from disk
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("shader.wgsl"))),
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
                targets: &[Some(swapchain_format.into())],
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        });
    
        let mut config = surface
            .get_default_config(&adapter, size.width, size.height)
            .unwrap();
    
        surface.configure(&device, &config);
    
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
                                rpass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint16);
                                rpass.draw_indexed(0..num_indices, 0, 0..1);
                            }
    
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

