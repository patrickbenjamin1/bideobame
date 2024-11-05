use pollster;
use std::sync::Arc;
use winit::application::ApplicationHandler;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowAttributes};

use super::state;

pub struct App {}

impl App {
    pub fn init() {
        // define event loop
        let event_loop = EventLoop::new().unwrap();

        event_loop.set_control_flow(ControlFlow::Poll);

        // define app window
        let mut app_window = AppWindow::default();

        // run the app - will return once the app is closed
        let result = event_loop.run_app(&mut app_window);

        match result {
            Ok(_) => println!("App exited successfully"),
            Err(e) => println!("App exited with error: {:?}", e),
        }
    }
}

#[derive(Default)]
pub struct AppWindow {
    window: Option<Window>,
    state: Option<state::State<'static>>,
}

impl ApplicationHandler for AppWindow {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = Some(
            event_loop
                .create_window(Window::default_attributes())
                .unwrap(),
        );

        self.window = window;

        // let state = pollster::block_on(state::State::new(window.as_ref().unwrap()));
        // self.state = Some(state);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event {
            winit::event::WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            _ => {}
        }
    }
}
