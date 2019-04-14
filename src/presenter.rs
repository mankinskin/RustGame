/*
 * presenter.rs
 * An extra wrapper around winit and voodoo, providing
 * a simple, low boilerplate interface for a vulkan window
 */

use vulkan;
use vulkan::{SwapchainComponents};

use voodoo_winit::winit::{Window, EventsLoop};
use voodoo::{Result as VdResult, ApplicationInfo, Instance, SurfaceKhr, Extent2d, Device,
            DescriptorSetLayout, DescriptorSet, PipelineLayout, CommandPool, DescriptorPool,
            SwapchainKhr, Sampler, CommandBuffer, Buffer, DeviceMemory,
            CommandBufferHandle, Image, ImageView};

use smallvec::SmallVec;

use vulkan::{Vertex};

// RESOURCE DATA
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

pub struct Presenter {
    pub instance: Instance,
    pub events_loop: EventsLoop,
    pub window: Window,
    pub surface: SurfaceKhr,
    pub device: Device,
    pub descriptor_set_layout: DescriptorSetLayout,
    pub descriptor_sets: SmallVec<[DescriptorSet; 8]>,
    pub pipeline_layout: PipelineLayout,
    pub command_pool: CommandPool,
    pub descriptor_pool: DescriptorPool,
    pub texture_sampler: Sampler,
    pub uniform_buffer: Buffer,
    pub uniform_buffer_memory: DeviceMemory,
    pub swapchain: Option<SwapchainKhr>,
    pub swapchain_components: Option<vulkan::SwapchainComponents>,
    pub command_buffers: Option<SmallVec<[CommandBuffer; 16]>>,
    pub command_buffer_handles: Option<SmallVec<[CommandBufferHandle; 16]>>,
    vert_shader_code: Vec<u32>,
    frag_shader_code: Vec<u32>,
    pub texture_image: Image,
    pub texture_image_memory: DeviceMemory,
    pub texture_image_view: ImageView,
    vertices: Vec<vulkan::Vertex>,
    indices: Vec<u32>,
    vertex_buffer: Buffer,
    pub vertex_buffer_memory: DeviceMemory,
    index_buffer: Buffer,
    pub index_buffer_memory: DeviceMemory,
}
fn window_extent(window: &Window) -> Extent2d {
        let dims = window.get_inner_size().unwrap();
        Extent2d::builder()
            .height(dims.1 as u32)
            .width(dims.0 as u32)
            .build()
}

impl Presenter {
    pub fn new(info: ApplicationInfo<'static>) -> Presenter {

        // Begin Initialization
        //
        let window_name = info.application_name()
            .to_str().unwrap().to_string();
        // Window EventsLoop
        let events_loop = EventsLoop::new();
        let window = vulkan::init_window(window_name,
                                         &events_loop).unwrap();
        let extent = window_extent(&window);

        // Vulkan instance object
        let instance = vulkan::init_instance(&info).unwrap();

        // Window Surface
        let surface = voodoo_winit::create_surface(instance.clone(),
                                                   &window).unwrap();

        // A physical Device (first detected)
        let physical_device = vulkan::choose_physical_device(&instance,
                                                             &surface).unwrap();
        // virtual Device
        let device = vulkan::create_device(&surface,
                                           physical_device).unwrap();

        let command_pool = vulkan::create_command_pool(device.clone(),
                                                       &surface).unwrap();

        let descriptor_pool = vulkan::create_descriptor_pool(device.clone()).unwrap();

        let descriptor_set_layout = vulkan::create_descriptor_set_layout(device.clone()).unwrap();

        let pipeline_layout = vulkan::create_pipeline_layout(device.clone(),
                                                             Some(&descriptor_set_layout)).unwrap();
        let texture_sampler = vulkan::create_texture_sampler(device.clone()).unwrap();

        let (depth_image, depth_image_memory, depth_image_view) =
            vulkan::create_depth_resources(&device,
                                           &command_pool,
                                           extent.clone()).unwrap();

        let (uniform_buffer, uniform_buffer_memory) =
            vulkan::create_uniform_buffer(&device,
                                          &command_pool,
                                          extent.clone()).unwrap();

        // Surface swapchain
        let swapchain = vulkan::create_swapchain(surface.clone(),
                                                 device.clone(),
                                                 Some(extent.clone()),
                                                 None).unwrap();

        let image_views = vulkan::create_image_views(&swapchain).unwrap();

        // RESOURCES

        let vert_shader_code = voodoo::util::read_spir_v_file(VERT_SHADER_PATH).unwrap();
        let frag_shader_code = voodoo::util::read_spir_v_file(FRAG_SHADER_PATH).unwrap();

        // let (vertices, indices) = load_model(&device, &Path::new(MODEL_PATH)).unwrap();
        let vertices = VERTICES[..].to_owned();

        let indices = INDICES[..].to_owned();

        let (texture_image, texture_image_memory) =
            vulkan::create_texture_image(&device,
                                         &command_pool,
                                         TEXTURE_PATH).unwrap();

        let (vertex_buffer, vertex_buffer_memory) =
            vulkan::create_vertex_buffer(&device,
                                         &command_pool,
                                         &vertices).unwrap();

        let (index_buffer, index_buffer_memory) =
            vulkan::create_index_buffer(&device,
                                        &command_pool,
                                        &indices).unwrap();
        // -- End Resources
        let render_pass = vulkan::create_render_pass(device.clone(), swapchain.image_format()).unwrap();

        let graphics_pipeline = vulkan::create_graphics_pipeline(device.clone(), &pipeline_layout,
            &render_pass, extent.clone(), &vert_shader_code, &frag_shader_code).unwrap();


        let framebuffers = vulkan::create_framebuffers(&device, &render_pass,
            &image_views, &depth_image_view, extent.clone()).unwrap();

        let texture_image_view =
            vulkan::create_texture_image_view(device.clone(),
            &texture_image).unwrap();

        let swapchain_components = SwapchainComponents {
            image_views,
            render_pass,
            graphics_pipeline,
            depth_image,
            depth_image_memory,
            depth_image_view,
            framebuffers,
        };
        let descriptor_sets =
            vulkan::create_descriptor_sets(&descriptor_set_layout,
                                           &descriptor_pool,
                                           &uniform_buffer,
                                           &texture_image_view,
                                           &texture_sampler).unwrap();

        let command_buffers =
            vulkan::create_command_buffers(&device,
                                           &command_pool,
                                           &swapchain_components.render_pass,
                                           &swapchain_components.graphics_pipeline,
                                           &swapchain_components.framebuffers,
                                           &extent,
                                           &vertex_buffer,
                                           &index_buffer,
                                           vertices.len() as u32,
                                           vertices.len() as u32,
                                           &pipeline_layout,
                                           descriptor_sets[0].clone()).unwrap();

        let command_buffer_handles: SmallVec<[CommandBufferHandle; 16]> =
            command_buffers.iter().map(|cb| cb.handle()).collect();


        Presenter {
            instance,
            window,
            events_loop,
            surface,
            device,
            descriptor_set_layout,
            descriptor_sets,
            pipeline_layout,
            command_pool,
            descriptor_pool,
            texture_sampler,
            uniform_buffer,
            uniform_buffer_memory,
            command_buffer_handles: Some(command_buffer_handles),
            swapchain: Some(swapchain),
            swapchain_components: Some(swapchain_components),
            command_buffers: Some(command_buffers),
            vert_shader_code,
            frag_shader_code,
            texture_image,
            texture_image_memory,
            texture_image_view,
            vertices,
            indices,
            vertex_buffer,
            vertex_buffer_memory,
            index_buffer,
            index_buffer_memory,
        }
    }
    pub fn extent(&self) -> Extent2d {
        window_extent(&self.window)
    }

    fn cleanup_swapchain(&mut self) {
        self.swapchain = None;
        self.swapchain_components = None;
        self.command_buffers = None;
    }

    pub fn recreate_swapchain(&mut self) -> VdResult<()> {
        self.device.wait_idle();

        let extent = self.extent();
        let swapchain = vulkan::create_swapchain(self.surface.clone(),
                                                 self.device.clone(),
                                                 Some(extent.clone()),
                                                 self.swapchain.as_ref().take()).unwrap();

        self.cleanup_swapchain();

        let image_views = vulkan::create_image_views(&swapchain).unwrap();
        let render_pass = vulkan::create_render_pass(self.device.clone(),
            swapchain.image_format()).unwrap();

        let graphics_pipeline = vulkan::create_graphics_pipeline(self.device.clone(),
            &self.pipeline_layout, &render_pass,
            extent.clone(), &self.vert_shader_code, &self.frag_shader_code).unwrap();

        let (depth_image, depth_image_memory, depth_image_view) = vulkan::create_depth_resources(
            &self.device, &self.command_pool, extent.clone()).unwrap();

        let framebuffers = vulkan::create_framebuffers(&self.device,
            &render_pass, &image_views,
            &depth_image_view, extent.clone()).unwrap();

        let command_buffers =
            vulkan::create_command_buffers(&self.device,
                                           &self.command_pool,
                                           &render_pass,
                                           &graphics_pipeline,
                                           &framebuffers,
                                           &extent,
                                           &self.vertex_buffer,
                                           &self.index_buffer,
                                           self.vertices.len() as u32,
                                           self.indices.len() as u32,
                                           &self.pipeline_layout,
                                           self.descriptor_sets[0].clone()).unwrap();

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

}
