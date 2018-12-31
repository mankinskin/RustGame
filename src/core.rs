/*
 * core.rs
 */
use input;
use winit;
use window::Window;

pub struct Application {
    pub window: Window
}

impl Application {
    pub fn new() -> Application {
        input::init();
        Application {
            window: Window::new()
        }
    }
}

pub fn frameloop(app: &mut Application) {
    app.window.events_loop.run_forever(frame);
}

fn frame(event: winit::Event) -> winit::ControlFlow {
    input::update(event)
}

pub fn cleanup() {
    println!("Farewell cruel world!");
}
