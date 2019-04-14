/*
 * app.rs
 * The core functionality of the application.
 * Kicking everything off and calling the last function
 */
use vulkan;
use input;
use presenter::{Presenter};

use std::ffi::CString;

use std::ops::Drop;

use voodoo_winit::winit::{ControlFlow};
use std::mem;
use std::time;

use cgmath::{Matrix3, Matrix4};

use voodoo::{Result as VdResult, ApplicationInfo, MemoryMapFlags};


lazy_static! {
    pub static ref APP_NAME: CString = CString::new("My App").unwrap();
}


pub struct App {
    pub info: ApplicationInfo<'static>,
    presenter: Presenter,
    start_time: time::Instant,
}

impl App {

    fn init_info(name: &'static CString) -> ApplicationInfo<'static> {
        ApplicationInfo::builder()
            .application_name(name)
            .application_version((1, 0, 0))
            .api_version((1, 0, 0))
            .build()
    }

    pub fn new() -> VdResult<App> {
        let info = Self::init_info(&APP_NAME);

        let presenter = Presenter::new(info.clone());


        let start_time = time::Instant::now();


        Ok(App {
            info,
            presenter: presenter,
            start_time,
        })
    }

    fn update_uniform_buffer(&mut self) -> VdResult<()> {
        let current_time = time::Instant::now();
        let elapsed = current_time.duration_since(self.start_time);
        let time = elapsed.as_secs() as f32 + (elapsed.subsec_nanos() as f32 * 1e-9);

        let extent = self.presenter.swapchain.as_ref().unwrap().extent().clone();

        let mut proj = cgmath::perspective(cgmath::Rad(45.0f32.to_radians()),
            extent.width() as f32 / extent.height() as f32, 0.1, 10.0);

        let view = cgmath::Matrix4::look_at(cgmath::Point3::new(2.0, 2.0, 2.0),
            cgmath::Point3::new(0.0, 0.0, 0.0), cgmath::Vector3::new(0.0, 0.0, 1.0));

        let scale = cgmath::Matrix4::from_scale(1.5);
        proj[1][1] *= -1.0;

        let rotation = Matrix3::from_angle_z(cgmath::Rad(time)) *
            Matrix3::from_angle_x(cgmath::Rad(time / 2.0));

        let model = Matrix4::from(rotation);

        let ubo = vulkan::UniformBufferObject {
            model: model.into(),
            view: (view * scale).into(),
            proj: proj.into(),
        };

        let mut data = unsafe {
            self.presenter.uniform_buffer_memory.map(0, mem::size_of::<vulkan::UniformBufferObject>() as u64,
                MemoryMapFlags::empty()).unwrap()
        };
        data.copy_from_slice(&[ubo]);
        self.presenter.uniform_buffer_memory.unmap(data);

        Ok(())
    }


    pub fn main_loop(&mut self) -> VdResult<()> {
        let mut exit = false;

        loop {
            self.presenter.events_loop.poll_events(|event| {
                // Add support for window resizing
                match input::update(event) {
                    ControlFlow::Break => { exit = true; },
                    _ => ()
                }
            });

            if exit { break; }

            self.update_uniform_buffer().unwrap();
            self.presenter.draw_frame().unwrap();
        }

        self.presenter.device.wait_idle();
        Ok(())
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
