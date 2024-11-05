use wgpu;
use winit::event::WindowEvent;
use winit::window::Window;

/// singleton state object that holds the wgpu device, queue, and surface
pub struct State<'a> {
    surface: wgpu::Surface<'a>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,

    // from winit
    window: &'a Window,
    render_pipeline: wgpu::RenderPipeline,
}

impl<'window> State<'window> {
    // Creating some of the wgpu types requires async code
    pub async fn new(window: &'window Window) -> State<'window> {
        // get the size from the winit window
        let size = window.inner_size();

        // create an instance
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            // use Vulkan backend
            backends: if cfg!(target_os = "macos") || cfg!(target_os = "ios") {
                wgpu::Backends::METAL
            } else {
                wgpu::Backends::VULKAN
            },
            ..Default::default()
        });

        // define surface
        let surface = instance.create_surface(window).unwrap();

        // request adapter
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
                ..Default::default()
            })
            .await
            .unwrap();

        // get device and queue from adapter
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor::default(), None)
            .await
            .unwrap();

        // get surface capabilities from adapter
        let surface_capabilities = surface.get_capabilities(&adapter);

        // get the surface format from the capabilities
        let surface_format = surface_capabilities
            .formats
            .iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_capabilities.formats[0]);

        // define the surface configuration
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_capabilities.present_modes[0],
            alpha_mode: surface_capabilities.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        surface.configure(&device, &config);

        // load shader entry point
        let shader = Self::load_shader(&device, include_str!("../shaders/shader.wgsl"));

        // create render pipeline layout
        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            });

        // create render pipeline
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw, // 2.
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None, // 1.
            multisample: wgpu::MultisampleState {
                count: 1,                         // 2.
                mask: !0,                         // 3.
                alpha_to_coverage_enabled: false, // 4.
            },
            multiview: None, // 5.
            cache: None,     // 6.
        });

        // assign the configuration to the surface
        Self {
            surface,
            device,
            queue,
            config,
            size,
            window,
            render_pipeline,
        }
    }

    fn load_shader(device: &wgpu::Device, path: &str) -> wgpu::ShaderModule {
        return device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(path.into()),
        });
    }

    /// Get a reference to the window associated with the state
    pub fn window(&self) -> &Window {
        &self.window
    }

    /// Resize the gwpu surface to reflect a new size
    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        // early return to avoid zero size - should be impossible, but adding this to avoid panics
        if new_size.width == 0 || new_size.height == 0 {
            return;
        }

        // define the new size in the state
        self.size = new_size;
        self.config.width = new_size.width;
        self.config.height = new_size.height;
        self.surface.configure(&self.device, &self.config);
    }

    fn input(&mut self, event: &WindowEvent) -> bool {
        todo!()
    }

    /// handle a window event - returns a boolean indicating whether state handled the event
    /// @note I've removed this for now in favour of keeping event handling at the App level, with the intention of calling functions on here and passing
    /// events to the game event system

    // fn update(&mut self) {
    //     return false;
    // }

    /// render the current state to a frame
    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        // get the current texture from the surface
        let output = self.surface.get_current_texture()?;

        // create a view from the texture
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        // create encoder - this is a command buffer for the gpu
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        // do the following in a block so that the borrow of encoder is dropped before we submit it
        {
            // start a render pass
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            render_pass.set_pipeline(&self.render_pipeline);

            // draw a cool triangle
            render_pass.draw(0..3, 0..1);

            // draw another cool triangle
            render_pass.draw(4..7, 0..1);
        }

        // submit the rendered frame to the queue
        self.queue.submit(std::iter::once(encoder.finish()));

        // render the frame
        output.present();

        Result::Ok(())
    }
}
