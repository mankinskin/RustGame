/*
 * input.rs
 */
extern crate winit;

pub fn init() {

}

fn handle_key_press(key: winit::ScanCode) -> winit::ControlFlow {
    println!("Pressed {}", key);
    winit::ControlFlow::Continue
}

fn handle_key_release(key: winit::ScanCode) -> winit::ControlFlow {
    println!("Released {}", key);
    winit::ControlFlow::Continue
}

fn handle_keyboard_input(input: winit::KeyboardInput) -> winit::ControlFlow {
    match input {
        winit::KeyboardInput {
            scancode,
            state: winit::ElementState::Pressed,
            .. } => handle_key_press(scancode),
        winit::KeyboardInput {
            scancode,
            state: winit::ElementState::Released,
            .. } => handle_key_release(scancode)
    }
}

fn handle_window_event(event: winit::WindowEvent) -> winit::ControlFlow {
    match event {
        winit::WindowEvent::CloseRequested => winit::ControlFlow::Break,
        winit::WindowEvent::KeyboardInput {
            input,
            ..
        } => handle_keyboard_input(input),
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
