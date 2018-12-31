/*
 * main.rs
 */
mod core;
mod input;
mod window;
extern crate winit;

fn main() {
    let mut app = core::Application::new();
    core::frameloop(&mut app);
    core::cleanup();
}
