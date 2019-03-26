/*
 * core.rs
 * The core functionality of the application.
 * Kicking everything off and calling the last function
 */
use input;
use winit;
use vulkan;
use std::sync::Arc;


use vulkano::instance::{Instance};
use vulkano::device::{Device, DeviceExtensions, Queue};
use vulkano::buffer::{CpuAccessibleBuffer, BufferUsage};
use vulkano::swapchain::{Swapchain, PresentMode, SurfaceTransform, SwapchainCreationError, AcquireError};
use vulkano::swapchain::Surface;
use vulkano::swapchain;
use vulkano::image::SwapchainImage;
use vulkano::pipeline::GraphicsPipeline;
use vulkano::pipeline::viewport::Viewport;
use vulkano::framebuffer::{Subpass, Framebuffer, FramebufferAbstract, RenderPassAbstract};
use vulkano::command_buffer::{AutoCommandBufferBuilder, DynamicState};
use vulkano::sync::{GpuFuture, FlushError};
use vulkano::sync;

use winit::{EventsLoop, WindowBuilder};
use vulkano_win::VkSurfaceBuild;

pub struct Application {
    pub events_loop: EventsLoop,
    pub surface: Arc<Surface<winit::Window>>,
    pub vinstance: &'static Arc<Instance>,
    pub device: Arc<Device>,
    pub queue: Arc<Queue>,
    pub swapchain: Arc<Swapchain<winit::Window>>,
    pub images: Vec<Arc<SwapchainImage<winit::Window>>>
}

impl Application {
    pub fn new(vinstance: &'static Arc<Instance>) -> Application {
        let events_loop = EventsLoop::new();
        let surface = WindowBuilder::new().build_vk_surface(&events_loop, vinstance.clone()).unwrap();
        let pdev = vulkan::new_physical_device(&vinstance);
        // get supported queues
        let queue_fam = pdev.queue_families().find(|&q| {
            q.supports_graphics() && surface.is_supported(q).unwrap_or(false)
        }).unwrap();
        // init device
        let device_ext = DeviceExtensions { khr_swapchain: true, .. DeviceExtensions::none() };
        let (device, mut queues) =
            Device::new(pdev, pdev.supported_features(),
                        &device_ext, [(queue_fam, 0.5)].iter().cloned()
                    ).unwrap();
        let queue = queues.next().unwrap();
        // build swapchain
        let (swapchain, images) = {
            let caps = surface.capabilities(pdev).unwrap();
            let usage = caps.supported_usage_flags;
            let alpha = caps.supported_composite_alpha.iter().next().unwrap();
            let format = caps.supported_formats[0].0;
            let initial_dim = surface_get_dimensions(&surface);
            // create swapchain
            Swapchain::new( device.clone(), surface.clone(), caps.min_image_count,
                            format, initial_dim, 1, usage, &queue,
                            SurfaceTransform::Identity, alpha, PresentMode::Fifo, true, None).unwrap()
        };
        Application {
            events_loop: events_loop,
            surface: surface,
            vinstance: vinstance,
            device: device,
            swapchain: swapchain,
            images: images,
            queue: queue
        }
    }

    pub fn print_info(&self) {
        println!("Using Device: {} (type: {:?})",
                 self.device.physical_device().name(), self.device.physical_device().ty());
    }
}

pub fn frameloop(app: &mut Application) {
        #[derive(Debug, Clone)]
        struct Vertex { position: [f32; 2] }
        impl_vertex!(Vertex, position);
    let vertex_buffer = 
        CpuAccessibleBuffer::from_iter(app.device.clone(),
                                        BufferUsage::all(), [
                                            Vertex { position: [-0.5, -0.25] },
                                            Vertex { position: [0.5, -0.25] },
                                            Vertex { position: [-0.6, 0.5] }
                                        ].iter().cloned()).unwrap();
    mod vs {
        vulkano_shaders::shader!{
            ty: "vertex",
            src: "
                #version 450

                layout(location = 0) in vec2 position;

                void main() {
                    gl_Position = vec4(position, 0.0, 1.0);
                }"
        }
    }
    mod fs {
        vulkano_shaders::shader!{
            ty: "fragment",
            src: "
                #version 450

                layout(location = 0) out vec4 f_color;

                void main() {
                    f_color = vec4(1.0, 1.0, 0.0, 1.0);
                }"
        }
    }

    let vs = vs::Shader::load(app.device.clone()).unwrap();
    let fs = fs::Shader::load(app.device.clone()).unwrap();

    let render_pass = Arc::new(single_pass_renderpass!(
            app.device.clone(),
            attachments: {
                // custom name "color"
                color: {
                    load: Clear,
                    store: Store,
                    format: app.swapchain.format(),
                    samples: 1,
                }
            },
            pass: {
                // use "color" attachment as color attachment
                color: [color],
                depth_stencil: {} // none
            }).unwrap());

    // build rendering pipeline
    let pipeline = Arc::new( GraphicsPipeline::start()
        // indicate vertex buffer layout
        .vertex_input_single_buffer()
        // specify vertex shader entry point
        .vertex_shader(vs.main_entry_point(), ())
        // vertex buffer is a list of triangles
        .triangle_list()
        // use resizable viewport 
        .viewports_dynamic_scissors_irrelevant(1)
        .fragment_shader(fs.main_entry_point(), ())
        .render_pass(Subpass::from(render_pass.clone(), 0).unwrap())
        .build(app.device.clone()).unwrap());

    let mut dynamic_state = DynamicState { line_width: None, viewports: None, scissors: None };

    let mut framebuffers = window_size_dependent_setup(&app.images, render_pass.clone(), &mut dynamic_state);

    let mut recreate_swapchain = false;

    let mut previous_frame_end = Box::new(sync::now(app.device.clone())) as Box<GpuFuture>;

    loop {
        previous_frame_end.cleanup_finished();

        if recreate_swapchain {
            let dimensions = surface_get_dimensions(&app.surface);
            let (new_snapchain, new_images) =
                match app.swapchain.recreate_with_dimension(dimensions) {
                    Ok(r) => r,
                    Err(SwapchainCreationError::UnsupportedDimensions) => continue,
                    Err(err) => panic!("{:?}", err)
                };
            app.swapchain = new_snapchain;
            framebuffers = window_size_dependent_setup(&new_images, render_pass.clone(), &mut dynamic_state);
            recreate_swapchain = false;
        }

        // acquire next image from swapchain
        let (image_i, acquire_future) =
            match swapchain::acquire_next_image(app.swapchain.clone(), None) {
                Ok(r) => r,
                Err(AcquireError::OutOfDate) => {
                    recreate_swapchain = true;
                    continue;
                }
                Err(err) => panic!("{:?}", err)
            };
        let clear_values = vec!([0.0, 0.0, 0.2, 1.0].into());
        let command_buffer =
            AutoCommandBufferBuilder::primary_one_time_submit(app.device.clone(), app.queue.family()).unwrap()
            .begin_render_pass(framebuffers[image_i].clone(), false, clear_values) .unwrap()
            // we are now in the first subpass of the render pass
            .draw(pipeline.clone(), &dynamic_state, vertex_buffer.clone(), (), ()) .unwrap()
            // end render pass
            // call next_inline or next_secondary to issue another subpass
            .end_render_pass().unwrap()
            .build().unwrap();

        let future = previous_frame_end.join(acquire_future)
            .then_execute(app.queue.clone(), command_buffer).unwrap()
            // the color output is now expected to contain the triangle
            // but to make it visible, call *present*
            .then_swapchain_present(app.queue.clone(), app.swapchain.clone(), image_i)
            .then_signal_fence_and_flush();
        match future {
            Ok(future) => {
                previous_frame_end = Box::new(future) as Box<_>;
            }
            Err(FlushError::OutOfDate) => {
                recreate_swapchain = true;
                previous_frame_end = Box::new(sync::now(app.device.clone())) as Box<_>;
            }
            Err(e) => {
                println!("{:?}", e);
                previous_frame_end = Box::new(sync::now(app.device.clone())) as Box<_>;
            }
        }

        let mut done = false;
        app.events_loop.poll_events(|ev| {
            match input::update(ev) {
                winit::ControlFlow::Break => done = true,
                _ => ()
            }
        });
        if done { break; }
    }
}

fn surface_get_dimensions(surface: &Arc<Surface<winit::Window>>) -> [u32; 2] {
    if let Some(dimensions) = surface.window().get_inner_size() {
        let dimensions: (u32, u32) = dimensions.to_physical(surface.window().get_hidpi_factor()).into();
        [dimensions.0, dimensions.1]
    } else {
        // the window no longer exists, should exit!
        [0, 0] as [u32; 2]
    }
}
fn window_size_dependent_setup(
    images: &[Arc<SwapchainImage<winit::Window>>],
    render_pass: Arc<RenderPassAbstract + Send + Sync>,
    dynamic_state: &mut DynamicState
    ) -> Vec<Arc<FramebufferAbstract + Send + Sync>> {
    let dimensions = images[0].dimensions();

    let viewport = Viewport {
        origin: [0.0, 0.0],
        dimensions: [dimensions[0] as f32, dimensions[1] as f32],
        depth_range: 0.0 .. 1.0,
    };
    dynamic_state.viewports = Some(vec!(viewport));

    images.iter().map(|image| {
        Arc::new(
            Framebuffer::start(render_pass.clone())
                .add(image.clone()).unwrap()
                .build().unwrap()
        ) as Arc<FramebufferAbstract + Send + Sync>
    }).collect::<Vec<_>>()
}


pub fn cleanup() {

}
