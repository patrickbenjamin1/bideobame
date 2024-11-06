use crate::core::state::State;
use crate::utils::log;

use winit::event_loop::ControlFlow;
use winit::platform::modifier_supplement::KeyEventExtModifierSupplement;
use winit::window::WindowBuilder;

pub struct App {}

impl App {
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
        let mut state = State::new(&window).await;

        // run event loop
        let result = event_loop.run(move |event, event_loop_window_target| match event {
            // handle events
            winit::event::Event::WindowEvent {
                ref event,
                window_id,
                ..
            } => {
                match event {
                    // handle keyboard input events
                    winit::event::WindowEvent::KeyboardInput { event, .. } => {
                        match event.key_without_modifiers() {
                            // handle escape key
                            winit::keyboard::Key::Named(winit::keyboard::NamedKey::Escape) => {
                                println!("Escape key pressed");
                                event_loop_window_target.exit();
                            }

                            // handle other keys
                            _ => (),
                        }

                        // @todo dispatch key down event to game event system
                    }

                    // handle resize events
                    winit::event::WindowEvent::Resized(physical_size) => {
                        println!("Resized window to {:?}", physical_size);

                        // update wgpu surface with new size
                        // @todo debounce resize events as this will get expensive
                        state.resize(*physical_size);

                        // @todo dispatch resize event to game event system
                    }

                    // handle close events
                    winit::event::WindowEvent::CloseRequested => {
                        println!("Closing window");

                        event_loop_window_target.exit();
                    }

                    // handle redraw events by submitting them to render on state
                    winit::event::WindowEvent::RedrawRequested => {
                        state.window().request_redraw();

                        // render wgpu into winit window
                        match state.render() {
                            // we rendered successfully
                            Ok(_) => (),

                            // Reconfigure the surface if it's lost or outdated
                            Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                                state.resize(state.size())
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
        });

        match result {
            Ok(_) => println!("Event loop exited successfully"),
            Err(e) => println!("Error: {:?}", e),
        }
    }
}
