/*
 * input.rs
 */
extern crate winit;

pub fn init() {

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

struct Key {
    pub id: winit::VirtualKeyCode,
    pub state: winit::ElementState,
    pub modifiers: winit::ModifiersState
}

fn handle_key(key: Key) -> winit::ControlFlow {
    println!("{:?} {:?}", key.state, key.id);
    match key {
        Key {
            id: winit::VirtualKeyCode::Escape,
            state: winit::ElementState::Pressed,
            .. } => winit::ControlFlow::Break,
        _ => winit::ControlFlow::Continue
    }
}

fn handle_keyboard_input(input: winit::KeyboardInput) -> winit::ControlFlow {
    match input {
        winit::KeyboardInput {
            virtual_keycode: None,
            ..
        } => winit::ControlFlow::Continue,
        winit::KeyboardInput {
            virtual_keycode: Some(keycode),
            state,
            modifiers, ..
        } => handle_key(Key {
                            id: keycode,
                            state: state,
                            modifiers: modifiers
                        })
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

