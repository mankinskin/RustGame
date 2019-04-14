/*
 * presenter.rs
 * An extra wrapper around winit and voodoo, providing
 * a simple, low boilerplate interface for a vulkan window
 */

use vulkan;

use voodoo_winit::winit::{Window, EventsLoop};
use voodoo::{Instance, SurfaceKhr, Extent2d};

pub struct Presenter {
    pub window: Window,
    pub events_loop: EventsLoop,
    pub surface: SurfaceKhr
}

impl Presenter {
    pub fn new(instance: Instance) -> Presenter {
        let window_name = "Presenter Window".to_string();
        // Window EventsLoop
        let events_loop = EventsLoop::new();
        let window = vulkan::init_window(window_name, &events_loop).unwrap();

        // Window Surface
        let surface = voodoo_winit::create_surface(instance, &window).unwrap();

        Presenter {
            window: window,
            events_loop: events_loop,
            surface: surface,
        }
    }
    pub fn extent(&self) -> Extent2d {
        let dims = self.window.get_inner_size().unwrap();
        Extent2d::builder()
            .height(dims.1 as u32)
            .width(dims.0 as u32)
            .build()
    }
}
