/*
 * input.rs
 */
extern crate winit;

pub fn init() {

}

fn handle_window_event(event: winit::WindowEvent) -> winit::ControlFlow {
    match event {
        winit::WindowEvent::CloseRequested => winit::ControlFlow::Break,
        _ => winit::ControlFlow::Continue
    }
}

pub fn update(event: winit::Event) -> winit::ControlFlow {
    match event {
        winit::Event::WindowEvent {
            event,
            ..
        } => handle_window_event(event),
        _  => winit::ControlFlow::Continue
    }
}
