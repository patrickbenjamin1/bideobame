use crate::core::geometry;
use std::sync::{Arc, Mutex};
use wgpu;
use winit;

/// singleton state object that holds the wgpu device, queue, and surface

pub struct Renderer<'a> {
    // from wgpu
    surface: wgpu::Surface<'a>,
    device: Arc<Mutex<wgpu::Device>>,
    queue: Arc<Mutex<wgpu::Queue>>,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    render_pipeline: wgpu::RenderPipeline,

    // from winit
    window: &'a winit::window::Window,
}

impl<'window> Renderer<'window> {
    // Creating some of the wgpu types requires async code
    pub async fn new(window: &'window winit::window::Window) -> Renderer<'window> {
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

        // create render pipeline
        let render_pipeline = Self::init_render_pipeline(&device, &config);

        // create shareable device and queue
        let device = Arc::new(Mutex::new(device));
        let queue = Arc::new(Mutex::new(queue));

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

    fn init_render_pipeline(
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
    ) -> wgpu::RenderPipeline {
        // load shaders
        let vertex_shader = Self::load_shader(&device, include_str!("../shaders/vertex.wgsl"));
        let fragment_shader = Self::load_shader(&device, include_str!("../shaders/fragment.wgsl"));

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
                module: &vertex_shader,
                entry_point: Some("vs_main"), // Remove Some()
                buffers: &[geometry::Vertex::desc()],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &fragment_shader,
                entry_point: Some("fs_main"), // Remove Some()
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState {
                        color: wgpu::BlendComponent::REPLACE,
                        alpha: wgpu::BlendComponent::REPLACE,
                    }),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None, // Change this to None to see both sides
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
            cache: None,
        });

        return render_pipeline;
    }

    fn load_shader(device: &wgpu::Device, path: &str) -> wgpu::ShaderModule {
        return device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(path.into()),
        });
    }

    /// Get a reference to the window associated with the state
    pub fn window(&self) -> &winit::window::Window {
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
        self.surface
            .configure(&self.device.lock().unwrap(), &self.config);
    }

    /// handle a window event - returns a boolean indicating whether state handled the event
    /// @note I've removed this for now in favour of keeping event handling at the App level, with the intention of calling functions on here and passing
    /// events to the game event system

    // fn input(&mut self, event: &winit::event::WindowEvent) -> bool {
    //     todo!()
    // }

    // fn update(&mut self) {
    //     return false;
    // }

    /// render the current state to a frame
    // pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
    //     // The actual rendering is now handled by MeshRenderer system
    //     // This method just ensures the surface is properly configured
    //     if self.size.width == 0 || self.size.height == 0 {
    //         return Ok(());
    //     }
    //     Ok(())
    // }

    // accessors

    pub fn queue(&self) -> &Arc<Mutex<wgpu::Queue>> {
        &self.queue
    }

    pub fn device(&self) -> &Arc<Mutex<wgpu::Device>> {
        &self.device
    }

    pub fn surface(&self) -> &wgpu::Surface {
        &self.surface
    }

    pub fn render_pipeline(&self) -> &wgpu::RenderPipeline {
        &self.render_pipeline
    }
}
