/*
 * input.rs
 */
extern crate winit;

pub fn init() {

}

fn should_quit(event: winit::Event) -> bool {
    match event {
        winit::Event::WindowEvent {
            event: winit::WindowEvent::CloseRequested,
            .. } => true,
            _ => false
    }
}

pub fn update(app: &mut core::Application) {
    app.window.events_loop.run_forever(|event| {
        println!("{:?}", event);
        match event {
            _ if should_quit(event) => winit::ControlFlow::Break,
            _ => winit::ControlFlow::Continue
        }
    });
}
