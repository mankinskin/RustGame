/*
 * main.rs
 */
//mod core;
//mod input;

extern crate image;

#[macro_use]
extern crate lazy_static;
extern crate voodoo;
extern crate winit;

mod app;
mod input;

fn main() {
    let mut app = app::App::new();

    app.frameloop();
    //app.print_info();
    //core::frameloop(&mut app);
}
