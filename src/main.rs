/*
 * main.rs
 */
//mod core;
//mod input;

extern crate image;
extern crate smallvec;
extern crate ordered_float;
extern crate cgmath;
extern crate tobj;

#[macro_use]
extern crate lazy_static;
extern crate voodoo;
extern crate voodoo_winit;
extern crate winit;

mod app;
mod input;
mod vulkan;

fn main() {
    let app = app::App::new();

    app.unwrap().main_loop().unwrap();
}
