/*
 * core.rs
 */
use input;
use window::Window;

pub struct Application {
    pub window: Window
}

impl Application {
    pub fn new() -> Application {
        input::init();
        Application {
            window: Window::new()
        }
    }
}

pub fn frameloop(app: &mut Application) {
    input::update(app);
}

pub fn cleanup() {
    println!("Farewell cruel world!");
}
