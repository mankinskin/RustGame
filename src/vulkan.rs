/*
 * vulkan.rs
 */

use vulkano::instance::{Instance, PhysicalDevice};
use vulkano_win;
use std::sync::Arc;

pub fn new_vulkan_instance() -> Arc<Instance> {
    let ext = vulkano_win::required_extensions();
    Instance::new(None, &ext, None).unwrap()
}

pub fn new_physical_device<'a>(vinst: &'a Arc<Instance>) -> PhysicalDevice<'a> {
    PhysicalDevice::enumerate(vinst).next().unwrap()
}
