use crate::components::mesh_component;
use crate::core::game;
use crate::core::renderer::Renderer;
use crate::utils::log;
use std::sync::{Arc, Mutex};

use winit::event_loop::ControlFlow;
use winit::window::WindowBuilder;

pub struct App {}

impl App {
    /// Create a new instance of the App
    fn init_event_loop() -> winit::event_loop::EventLoop<()> {
        let event_loop = winit::event_loop::EventLoop::new().unwrap();

        return event_loop;
    }

    pub async fn run() {
        // init winit event loop
        let event_loop = Self::init_event_loop();

        // init window
        let window = WindowBuilder::new().build(&event_loop).unwrap();

        // set event loop to constantly poll
        event_loop.set_control_flow(ControlFlow::Poll);

        // define global wgpu state - holds device, queue, surface, etc.
        let mut renderer = Renderer::new(&window).await;

        // define world (storage for entities, components, and systems)
        let mut world = game::World::new();

        world.test_world();

        // run event loop
        let result = event_loop.run(move |event, event_loop_window_target| {
            // run systems
            world.run_systems(&mut renderer);

            // handle window events
            match event {
                // handle events
                winit::event::Event::WindowEvent {
                    ref event,
                    // window_id,
                    ..
                } => {
                    match event {
                        // handle keyboard input events
                        winit::event::WindowEvent::KeyboardInput { event, .. } => {
                            match event {
                                // handle key up events
                                winit::event::KeyEvent {
                                    state: winit::event::ElementState::Released,
                                    physical_key: key,
                                    ..
                                } => {
                                    println!("Key released: {:?}", key);

                                    match key {
                                        // handle escape key
                                        winit::keyboard::PhysicalKey::Code(
                                            winit::keyboard::KeyCode::Escape,
                                        ) => {
                                            println!("Escape key pressed, closing window");

                                            event_loop_window_target.exit();
                                        }

                                        // handle q key
                                        winit::keyboard::PhysicalKey::Code(
                                            winit::keyboard::KeyCode::KeyQ,
                                        ) => {
                                            renderer.geometry_manager().remove_at_mesh_index(0);
                                        }

                                        _ => (),
                                    }

                                    // @todo dispatch key up event to game event system
                                }

                                // handle key down events
                                winit::event::KeyEvent {
                                    state: winit::event::ElementState::Pressed,
                                    physical_key: key,
                                    ..
                                } => {
                                    println!("Key pressed: {:?}", key);

                                    // @todo dispatch key down event to game event system
                                }

                                _ => (),
                            }

                            // @todo dispatch key down event to game event system
                        }

                        // handle resize events
                        winit::event::WindowEvent::Resized(physical_size) => {
                            println!("Resized window to {:?}", physical_size);

                            // update wgpu surface with new size
                            // @todo debounce resize events as this will get expensive
                            renderer.resize(*physical_size);

                            // @todo dispatch resize event to game event system
                        }

                        // handle close events
                        winit::event::WindowEvent::CloseRequested => {
                            println!("Closing window");

                            event_loop_window_target.exit();
                        }

                        // handle redraw events by submitting them to render on state
                        winit::event::WindowEvent::RedrawRequested => {
                            renderer.window().request_redraw();

                            // render wgpu into winit window
                            match renderer.render() {
                                // we rendered successfully
                                Ok(_) => (),

                                // Reconfigure the surface if it's lost or outdated
                                Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                                    renderer.resize(renderer.size())
                                }

                                // The system is out of memory, we should probably quit
                                Err(wgpu::SurfaceError::OutOfMemory) => {
                                    log::error("OutOfMemory");
                                    event_loop_window_target.exit();
                                }

                                // This happens when the a frame takes too long to present
                                Err(wgpu::SurfaceError::Timeout) => log::warn("Surface timeout"),
                            }
                        }

                        _ => (),
                    }
                }
                _ => (),
            }
        });

        match result {
            Ok(_) => println!("Event loop exited successfully"),
            Err(e) => println!("Error: {:?}", e),
        }
    }
}
