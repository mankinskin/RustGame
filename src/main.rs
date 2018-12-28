mod core;
use core::*;

fn main() {
    let mut state = State::Startup;
    setup(&mut state);
    frameloop(&mut state);
    cleanup();
}
