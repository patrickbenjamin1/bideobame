use crate::core::game;
use crate::core::renderer::Renderer;

use std::time::{Duration, Instant};
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
        event_loop.set_control_flow(ControlFlow::wait_duration(Duration::from_millis(16))); // ~60 FPS

        // define global wgpu state - holds device, queue, surface, etc.
        let mut renderer = Renderer::new(&window).await;

        // define world (storage for entities, components, and systems)
        let mut world = game::World::new();

        world.test_world();

        let mut last_frame = Instant::now();

        // Add debug before each system run
        let result = event_loop.run(move |event, event_loop_window_target| {
            // calculate time since last frame
            let now = Instant::now();
            let delta_time = (now - last_frame).as_secs_f32();
            last_frame = now;

            // update world state to reflect time passed
            world.state().update(delta_time);

            // handle window events
            match event {
                winit::event::Event::AboutToWait { .. } => {
                    // run update systems
                    world.run_update_systems(&mut renderer);
                }

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
                                            // leaving this as an example
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
                            // Run draw systems only during redraw
                            world.run_draw_systems(&mut renderer);
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
