/*
 * input.rs
 */
use voodoo_winit::winit::{
    ControlFlow, ElementState, Event, KeyboardInput, ModifiersState, VirtualKeyCode, WindowEvent,
};

pub fn update(event: Event) -> ControlFlow {
    match event {
        Event::WindowEvent { event, .. } => handle_window_event(event),
        _ => ControlFlow::Continue,
    }
}

struct Key {
    pub id: VirtualKeyCode,
    pub state: ElementState,
    pub modifiers: ModifiersState,
}

fn handle_key(key: Key) -> ControlFlow {
    println!("{:?} {:?}", key.state, key.id);
    match key {
        Key {
            id: VirtualKeyCode::Escape,
            state: ElementState::Pressed,
            ..
        } => ControlFlow::Break,
        _ => ControlFlow::Continue,
    }
}

fn handle_keyboard_input(input: KeyboardInput) -> ControlFlow {
    match input {
        KeyboardInput {
            virtual_keycode: None,
            ..
        } => ControlFlow::Continue,
        KeyboardInput {
            virtual_keycode: Some(keycode),
            state,
            modifiers,
            ..
        } => handle_key(Key {
            id: keycode,
            state: state,
            modifiers: modifiers,
        }),
    }
}

fn handle_window_event(event: WindowEvent) -> ControlFlow {
    match event {
        WindowEvent::Closed => ControlFlow::Break,
        WindowEvent::KeyboardInput { input, .. } => handle_keyboard_input(input),
        _ => ControlFlow::Continue,
    }
}
