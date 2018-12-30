/*
 * input.rs
 */
use std::io;
use core;

pub fn init() {

}

pub fn update(state: &mut core::State) {
    let stdin = io::stdin();
    let mut buffer = String::new();
    match stdin.read_line(&mut buffer) {
        Ok(_) => println!("{}", buffer),
        _ => println!("No input.")
    }
    core::quit(state);
}
