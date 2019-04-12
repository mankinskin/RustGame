/*
 * app.rs
 * The core functionality of the application.
 * Kicking everything off and calling the last function
 */
use vulkan;
use input;

use std::ffi::CString;

use std::ops::Drop;

use voodoo_winit::winit::{Window, EventsLoop, ControlFlow};
use std::mem;
use std::time;

use smallvec::SmallVec;
use cgmath::{Matrix3, Matrix4};

use voodoo::{Result as VdResult, Instance, SurfaceKhr,
    SwapchainKhr, ImageView, PipelineLayout, CommandPool, ApplicationInfo, Semaphore,
    SemaphoreCreateFlags, MemoryMapFlags, ErrorKind, CallResult, Device, Extent2d,
    PipelineStageFlags, SubmitInfo, PresentInfoKhr, Image, DeviceMemory, Sampler,
    Buffer, DescriptorPool, DescriptorSet, CommandBuffer, CommandBufferHandle,
    DescriptorSetLayout};

use vulkan::{Vertex, SwapchainComponents};
lazy_static! {
    pub static ref APP_NAME: CString = CString::new("My App").unwrap();
}

const VERTICES: [Vertex; 8] =  [
    Vertex { pos: [-0.5, -0.5, 0.25], color: [1.0, 0.0, 0.0], tex_coord: [1.0, 0.0]},
    Vertex { pos: [0.5, -0.5, 0.25], color: [0.0, 1.0, 0.0], tex_coord: [0.0, 0.0] },
    Vertex { pos: [0.5, 0.5, 0.25], color: [0.0, 0.0, 1.0], tex_coord: [0.0, 1.0] },
    Vertex { pos: [-0.5, 0.5, 0.25], color: [1.0, 1.0, 1.0], tex_coord: [1.0, 1.0] },
    Vertex { pos: [-0.5, -0.5, -0.25], color: [1.0, 0.0, 0.0], tex_coord: [1.0, 0.0]},
    Vertex { pos: [0.5, -0.5, -0.25], color: [0.0, 1.0, 0.0], tex_coord: [0.0, 0.0] },
    Vertex { pos: [0.5, 0.5, -0.25], color: [0.0, 0.0, 1.0], tex_coord: [0.0, 1.0] },
    Vertex { pos: [-0.5, 0.5, -0.25], color: [1.0, 1.0, 1.0], tex_coord: [1.0, 1.0] },
];
const INDICES: [u32; 12] = [
    0, 1, 2, 2, 3, 0,
    4, 5, 6, 6, 7, 4
];

// Resource Paths
// static MODEL_PATH: &str = "/src/shared_assets/models/chalet.obj";
static VERT_SHADER_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"),
    "/shaders/vert.spv");
static FRAG_SHADER_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"),
    "/shaders/frag.spv");

static TEXTURE_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"),
    "/images/hello.jpg");

pub struct App {
    pub info: ApplicationInfo<'static>,
    pub instance: Instance,
    window: Window,
    events_loop: EventsLoop,
    device: Device,
    surface: SurfaceKhr,
    pub descriptor_set_layout: DescriptorSetLayout,
    pipeline_layout: PipelineLayout,
    vert_shader_code: Vec<u32>,
    frag_shader_code: Vec<u32>,
    command_pool: CommandPool,
    pub texture_image: Image,
    pub texture_image_memory: DeviceMemory,
    pub texture_image_view: ImageView,
    pub texture_sampler: Sampler,
    vertices: Vec<vulkan::Vertex>,
    indices: Vec<u32>,
    vertex_buffer: Buffer,
    pub vertex_buffer_memory: DeviceMemory,
    index_buffer: Buffer,
    pub index_buffer_memory: DeviceMemory,
    pub uniform_buffer: Buffer,
    uniform_buffer_memory: DeviceMemory,
    pub descriptor_pool: DescriptorPool,
    descriptor_sets: SmallVec<[DescriptorSet; 8]>,
    image_available_semaphore: Semaphore,
    render_finished_semaphore: Semaphore,
    start_time: time::Instant,
    swapchain: Option<SwapchainKhr>,
    swapchain_components: Option<vulkan::SwapchainComponents>,
    command_buffers: Option<SmallVec<[CommandBuffer; 16]>>,
    command_buffer_handles: Option<SmallVec<[CommandBufferHandle; 16]>>,
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
        let app_name = APP_NAME.to_str().unwrap().to_string();
        let info = Self::init_info(&APP_NAME);
        // Vulkan instance object
        let instance = vulkan::init_instance(&info).unwrap();
        // Window EventsLoop
        let events_loop = EventsLoop::new();

        let window = vulkan::init_window(app_name, &events_loop).unwrap();
        // Window Surface
        let surface = voodoo_winit::create_surface(instance.clone(), &window).unwrap();
        // A physical Device (first detected)
        let physical_device = vulkan::choose_physical_device(&instance, &surface).unwrap();
        // virtual Device
        let device = vulkan::create_device(&surface, physical_device).unwrap();
        //
        let swapchain = vulkan::create_swapchain(surface.clone(), device.clone(), None, None).unwrap();
        let image_views = vulkan::create_image_views(&swapchain).unwrap();
        let render_pass = vulkan::create_render_pass(device.clone(), swapchain.image_format()).unwrap();
        let descriptor_set_layout = vulkan::create_descriptor_set_layout(device.clone()).unwrap();
        let pipeline_layout = vulkan::create_pipeline_layout(device.clone(),
            Some(&descriptor_set_layout)).unwrap();
        let vert_shader_code = voodoo::util::read_spir_v_file(VERT_SHADER_PATH).unwrap();
        let frag_shader_code = voodoo::util::read_spir_v_file(FRAG_SHADER_PATH).unwrap();
        let graphics_pipeline = vulkan::create_graphics_pipeline(device.clone(), &pipeline_layout,
            &render_pass, swapchain.extent().clone(), &vert_shader_code, &frag_shader_code).unwrap();
        let command_pool = vulkan::create_command_pool(device.clone(), &surface).unwrap();
        let (depth_image, depth_image_memory, depth_image_view) = vulkan::create_depth_resources(&device,
            &command_pool, swapchain.extent().clone()).unwrap();
        let framebuffers = vulkan::create_framebuffers(&device, &render_pass,
            &image_views, &depth_image_view, swapchain.extent().clone()).unwrap();
        let (texture_image, texture_image_memory) = vulkan::create_texture_image(&device,
            &command_pool, TEXTURE_PATH).unwrap();
        let texture_image_view = vulkan::create_texture_image_view(device.clone(),
            &texture_image).unwrap();
        let texture_sampler = vulkan::create_texture_sampler(device.clone()).unwrap();

        // let (vertices, indices) = load_model(&device, &Path::new(MODEL_PATH)).unwrap();
        let vertices = VERTICES[..].to_owned();
        let indices = INDICES[..].to_owned();
        let (vertex_buffer, vertex_buffer_memory) = vulkan::create_vertex_buffer(&device, &command_pool,
            &vertices).unwrap();
        let (index_buffer, index_buffer_memory) = vulkan::create_index_buffer(&device, &command_pool,
            &indices).unwrap();
        let (uniform_buffer, uniform_buffer_memory) = vulkan::create_uniform_buffer(&device,
            &command_pool, swapchain.extent().clone()).unwrap();
        let descriptor_pool = vulkan::create_descriptor_pool(device.clone()).unwrap();
        let descriptor_sets = vulkan::create_descriptor_sets(&descriptor_set_layout,
            &descriptor_pool, &uniform_buffer, &texture_image_view, &texture_sampler).unwrap();
        let command_buffers = vulkan::create_command_buffers(&device, &command_pool, &render_pass,
            &graphics_pipeline, &framebuffers, swapchain.extent(),
            &vertex_buffer, &index_buffer,
            vertices.len() as u32, vertices.len() as u32, &pipeline_layout,
            descriptor_sets[0].clone()).unwrap();
        let image_available_semaphore = Semaphore::new(device.clone(),
            SemaphoreCreateFlags::empty()).unwrap();
        let render_finished_semaphore = Semaphore::new(device.clone(),
            SemaphoreCreateFlags::empty()).unwrap();
        let start_time = time::Instant::now();

        let swapchain_components = SwapchainComponents {
            image_views: image_views,
            render_pass: render_pass,
            graphics_pipeline: graphics_pipeline,
            depth_image,
            depth_image_memory,
            depth_image_view,
            framebuffers: framebuffers,
        };

        let command_buffer_handles = command_buffers.iter().map(|cb| cb.handle()).collect();

        Ok(App {
            info,
            instance,
            window: window,
            events_loop: events_loop,
            device: device,
            surface: surface,
            descriptor_set_layout,
            pipeline_layout,
            vert_shader_code,
            frag_shader_code,
            command_pool,
            texture_image,
            texture_image_memory,
            texture_image_view,
            texture_sampler,
            vertices: vertices,
            indices: indices,
            vertex_buffer,
            vertex_buffer_memory,
            index_buffer,
            index_buffer_memory,
            uniform_buffer,
            uniform_buffer_memory,
            descriptor_pool,
            descriptor_sets,
            image_available_semaphore,
            render_finished_semaphore,
            start_time,
            swapchain: Some(swapchain),
            swapchain_components: Some(swapchain_components),
            command_buffers: Some(command_buffers),
            command_buffer_handles: Some(command_buffer_handles),
        })
    }

    fn cleanup_swapchain(&mut self) {
        self.swapchain = None;
        self.swapchain_components = None;
        self.command_buffers = None;
    }

    fn recreate_swapchain(&mut self, current_extent: Extent2d) -> VdResult<()> {
        self.device.wait_idle();

        let swapchain = vulkan::create_swapchain(self.surface.clone(), self.device.clone(),
            Some(current_extent), self.swapchain.as_ref().take()).unwrap();

        self.cleanup_swapchain();

        let image_views = vulkan::create_image_views(&swapchain).unwrap();
        let render_pass = vulkan::create_render_pass(self.device.clone(),
            swapchain.image_format()).unwrap();
        let graphics_pipeline = vulkan::create_graphics_pipeline(self.device.clone(),
            &self.pipeline_layout, &render_pass,
            swapchain.extent().clone(), &self.vert_shader_code, &self.frag_shader_code).unwrap();
        let (depth_image, depth_image_memory, depth_image_view) = vulkan::create_depth_resources(
            &self.device, &self.command_pool, swapchain.extent().clone()).unwrap();
        let framebuffers = vulkan::create_framebuffers(&self.device,
            &render_pass, &image_views,
            &depth_image_view, swapchain.extent().clone()).unwrap();
        let command_buffers = vulkan::create_command_buffers(&self.device, &self.command_pool,
            &render_pass, &graphics_pipeline,
            &framebuffers, swapchain.extent(),
            &self.vertex_buffer, &self.index_buffer, self.vertices.len() as u32,
            self.indices.len() as u32, &self.pipeline_layout, self.descriptor_sets[0].clone()).unwrap();
        let command_buffer_handles = command_buffers.iter().map(|cb| cb.handle()).collect();

        self.swapchain = Some(swapchain);
        self.swapchain_components = Some(vulkan::SwapchainComponents {
            image_views: image_views,
            render_pass: render_pass,
            graphics_pipeline: graphics_pipeline,
            depth_image,
            depth_image_memory,
            depth_image_view,
            framebuffers: framebuffers,
        });
        self.command_buffers = Some(command_buffers);
        self.command_buffer_handles = Some(command_buffer_handles);

        Ok(())
    }

    fn update_uniform_buffer(&mut self) -> VdResult<()> {
        let current_time = time::Instant::now();
        let elapsed = current_time.duration_since(self.start_time);
        let time = elapsed.as_secs() as f32 + (elapsed.subsec_nanos() as f32 * 1e-9);

        let extent = self.swapchain.as_ref().unwrap().extent().clone();
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
            self.uniform_buffer_memory.map(0, mem::size_of::<vulkan::UniformBufferObject>() as u64,
                MemoryMapFlags::empty()).unwrap()
        };
        data.copy_from_slice(&[ubo]);
        self.uniform_buffer_memory.unmap(data);

        Ok(())
    }

    fn draw_frame(&mut self) -> VdResult<()> {
        let acquire_result = self.swapchain.as_ref().unwrap().acquire_next_image_khr(
            u64::max_value(), Some(&self.image_available_semaphore), None);
        let image_index = match acquire_result {
            Ok(idx) => idx,
            Err(res) => {
                if let ErrorKind::ApiCall(call_res, _fn_name) = res.kind {
                    if call_res == CallResult::ErrorOutOfDateKhr {
                        let dims = self.window.get_inner_size().unwrap();
                        self.recreate_swapchain(Extent2d::builder()
                            .height(dims.1 as u32)
                            .width(dims.0 as u32)
                            .build()).unwrap();
                        return Ok(());
                    } else {
                        panic!("Unable to present swap chain image");
                    }
                } else {
                    panic!("Unable to present swap chain image");
                }
            }
        };

        let wait_semaphores = [self.image_available_semaphore.handle()];
        let wait_stages = PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT;
        let signal_semaphores = [self.render_finished_semaphore.handle()];
        let command_buffer_handles = [self.command_buffer_handles.as_ref().unwrap()
            .get(image_index as usize).unwrap().clone()];

        let submit_info = SubmitInfo::builder()
            .wait_semaphores(&wait_semaphores[..])
            .wait_dst_stage_mask(&wait_stages)
            .command_buffers(&command_buffer_handles[..])
            .signal_semaphores(&signal_semaphores[..])
            .build();

        let queue = self.device.queue(0).unwrap();
        queue.submit(&[submit_info], None).unwrap();

        let swapchains = [self.swapchain.as_ref().unwrap().handle()];
        let image_indices = [image_index];

        let present_info = PresentInfoKhr::builder()
            .wait_semaphores(&signal_semaphores[..])
            .swapchains(&swapchains[..])
            .image_indices(&image_indices)
            .build();

        queue.present_khr(&present_info).unwrap();
        queue.wait_idle();

        Ok(())
    }

    pub fn main_loop(&mut self) -> VdResult<()> {
        let mut exit = false;
        let mut recreate_swap = false;
        let mut current_extent = self.swapchain.as_ref().unwrap().extent().clone();

        loop {
            self.events_loop.poll_events(|event| {
                match input::update(event) {
                    ControlFlow::Break => { exit = true; },
                    _ => ()
                }
            });

            if recreate_swap {
                self.recreate_swapchain(current_extent.clone()).unwrap();
                recreate_swap = false;
            };
            if exit { break; }

            self.update_uniform_buffer().unwrap();
            self.draw_frame().unwrap();
        }

        self.device.wait_idle();
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
