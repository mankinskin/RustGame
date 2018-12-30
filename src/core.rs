/*
 * core.rs
 */
use input;

pub enum State {
    Startup,
    Running,
    Quitting,
}

pub fn setup(state: &mut State) {
    println!("Hello world!");
    *state = State::Running;
    input::init();
}

pub fn frameloop(state: &mut State) {
    loop {
        match state {
            State::Quitting => break,
            _ => (),
        }
        input::update(state);
        println!("DumDi");
    }
}
pub fn cleanup() {
    println!("Farewell cruel world!");
}

pub fn quit(state: &mut State) {
    *state = State::Quitting;
}
