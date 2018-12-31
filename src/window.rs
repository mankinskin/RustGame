/*
 * window.rs
 */
use winit;

pub struct Window {
    pub window: winit::Window,
    pub events_loop: winit::EventsLoop
}

impl Window {
    pub fn new() -> Window {
        let events_loop = winit::EventsLoop::new();
        let window = winit::Window::new(&events_loop).unwrap();
        Window {
            events_loop: events_loop,
            window: window
        }
    }
}
