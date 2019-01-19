/*
 * main.rs
 */
mod core;
mod input;
mod vulkan;

extern crate winit;
extern crate image;
extern crate vulkano_win;
#[macro_use]
extern crate vulkano_shaders;
#[macro_use]
extern crate vulkano;
#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref vinstance: std::sync::Arc<vulkano::instance::Instance> =
        vulkan::new_vulkan_instance();
}

fn main() {
    let mut app = core::Application::new(&vinstance);
    app.print_info();
    core::frameloop(&mut app);
    core::cleanup();
}
