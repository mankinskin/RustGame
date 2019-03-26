/*
 * input.rs
 */
extern crate winit;
use winit::{Event, ControlFlow};

pub fn update(event: Event) -> ControlFlow {
    match event {
        Event::WindowEvent {
            event,
            ..
        } => handle_window_event(event),
        _  => ControlFlow::Continue
    }
}

struct Key {
    pub id: winit::VirtualKeyCode,
    pub state: winit::ElementState,
    pub modifiers: winit::ModifiersState
}

fn handle_key(key: Key) -> ControlFlow {
    println!("{:?} {:?}", key.state, key.id);
    match key {
        Key {
            id: winit::VirtualKeyCode::Escape,
            state: winit::ElementState::Pressed,
            .. } => ControlFlow::Break,
        _ => ControlFlow::Continue
    }
}

fn handle_keyboard_input(input: winit::KeyboardInput) -> ControlFlow {
    match input {
        winit::KeyboardInput {
            virtual_keycode: None,
            ..
        } => ControlFlow::Continue,
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

fn handle_window_event(event: winit::WindowEvent) -> ControlFlow {
    match event {
        winit::WindowEvent::CloseRequested => winit::ControlFlow::Break,
        winit::WindowEvent::KeyboardInput {
            input,
            ..
        } => handle_keyboard_input(input),
        _ => ControlFlow::Continue
    }
}

