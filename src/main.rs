/*
 * main.rs
 */
mod core;
mod input;

fn main() {
    let mut state = core::State::Startup;
    core::setup(&mut state);
    core::frameloop(&mut state);
    core::cleanup();
}
