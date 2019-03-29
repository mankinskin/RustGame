/*
 * app.rs
 * The core functionality of the application.
 * Kicking everything off and calling the last function
 */
use input;
use winit;

use std::ffi::CString;

use std::string::String;

use winit::{WindowBuilder, EventsLoop};
use voodoo::{Loader, Instance, ApplicationInfo};


type Window = (winit::Window, EventsLoop);

lazy_static! {
    pub static ref APP_NAME: CString = CString::new("My App").unwrap();
}

pub struct App {
    pub info: ApplicationInfo<'static>,
    pub instance: Instance,
    pub window: Window,
}

impl App {
    fn init_instance(info: &ApplicationInfo<'static>) -> Instance {
        let loader = Loader::new().unwrap();
        Instance::builder()
            .application_info(info)
            .enabled_extensions(&loader.enumerate_instance_extension_properties().unwrap())
            .build(loader).unwrap()
    }
    fn init_window(name: String) -> Window {
        let events_loop = EventsLoop::new();
        let window = WindowBuilder::new()
            .with_title(name)
            .build(&events_loop).unwrap();
        (window, events_loop)
    }

    fn init_info(name: &'static CString) -> ApplicationInfo<'static> {
        ApplicationInfo::builder()
            .application_name(name)
            .application_version((1, 0, 0))
            .api_version((1, 0, 0))
            .build()
    }
    pub fn new() -> App {
        let info = Self::init_info(&APP_NAME);
        let instance = Self::init_instance(&info);
        App {
            info,
            instance,
            window: Self::init_window(APP_NAME.to_str().unwrap().to_string()),
        }
    }

    pub fn frameloop(&mut self) {
        let mut done = false;
        loop {
            self.window.1.poll_events(|ev| {
                match input::update(ev) {
                    winit::ControlFlow::Break => done = true,
                    _ => ()
                }
            });
            if done { break; }
        }
    }
}
    //pub fn with_title(&mut self, title: Cstring) -> App {
    //    self.APP_NAME = title;
    //    self
    //}

    //pub fn print_info(&self) {
    //    println!("Using Device: {} (type: {:?})",
    //             self.device.physical_device().name(), self.device.physical_device().ty());
    //}


//fn surface_get_dimensions(surface: &Arc<Surface<winit::Window>>) -> [u32; 2] {
//    if let Some(dimensions) = surface.window().get_inner_size() {
//        let dimensions: (u32, u32) = dimensions.to_physical(surface.window().get_hidpi_factor()).into();
//        [dimensions.0, dimensions.1]
//    } else {
//        // the window no longer exists, should exit!
//        [0, 0] as [u32; 2]
//    }
//}
//
//fn window_size_dependent_setup(
//    images: &[Arc<SwapchainImage<winit::Window>>],
//    render_pass: Arc<RenderPassAbstract + Send + Sync>,
//    dynamic_state: &mut DynamicState
//    ) -> Vec<Arc<FramebufferAbstract + Send + Sync>> {
//    let dimensions = images[0].dimensions();
//
//    let viewport = Viewport {
//        origin: [0.0, 0.0],
//        dimensions: [dimensions[0] as f32, dimensions[1] as f32],
//        depth_range: 0.0 .. 1.0,
//    };
//    dynamic_state.viewports = Some(vec!(viewport));
//
//    images.iter().map(|image| {
//        Arc::new(
//            Framebuffer::start(render_pass.clone())
//                .add(image.clone()).unwrap()
//                .build().unwrap()
//        ) as Arc<FramebufferAbstract + Send + Sync>
//    }).collect::<Vec<_>>()
//}

