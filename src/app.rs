/*
 * app.rs
 * The core functionality of the application.
 * Kicking everything off and calling the last function
 */
use input;

use std::ffi::CString;

use std::ops::Drop;
use std::string::String;

use voodoo::{ApplicationInfo, Instance, Loader, SurfaceKhr};
use voodoo_winit::winit::{ControlFlow, EventsLoop, Window, WindowBuilder};

lazy_static! {
    pub static ref APP_NAME: CString = CString::new("My App").unwrap();
}

pub struct App {
    pub info: ApplicationInfo<'static>,
    pub instance: Instance,
    pub events_loop: EventsLoop,
    pub window: Window,
    pub surface: SurfaceKhr,
}

impl App {
    fn init_instance(info: &ApplicationInfo<'static>) -> Instance {
        let loader = Loader::new().unwrap();
        Instance::builder()
            .application_info(info)
            .enabled_extensions(&loader.enumerate_instance_extension_properties().unwrap())
            .build(loader)
            .unwrap()
    }
    fn init_window(name: String, events_loop: &EventsLoop) -> Window {
        WindowBuilder::new()
            .with_title(name)
            .build(events_loop)
            .unwrap()
    }

    fn init_info(name: &'static CString) -> ApplicationInfo<'static> {
        ApplicationInfo::builder()
            .application_name(name)
            .application_version((1, 0, 0))
            .api_version((1, 0, 0))
            .build()
    }

    pub fn new() -> App {
        let app_name = APP_NAME.to_str().unwrap().to_string();
        let info = Self::init_info(&APP_NAME);
        let instance = Self::init_instance(&info);
        let events_loop = EventsLoop::new();
        let window = Self::init_window(app_name, &events_loop);
        let surface = voodoo_winit::create_surface(instance.clone(), &window).unwrap();

        App {
            info,
            instance,
            events_loop,
            window,
            surface,
        }
    }

    pub fn frameloop(&mut self) {
        let mut done = false;
        loop {
            self.events_loop.poll_events(|ev| match input::update(ev) {
                ControlFlow::Break => done = true,
                _ => (),
            });
            if done {
                break;
            }
        }
    }
}

impl Drop for App {
    fn drop(&mut self) {
        println!("Goodbye.");
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
