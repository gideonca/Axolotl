extern crate glfw;

// use glfw::{Action, Context, Key};

use std::sync::Arc;

use vulkano::buffer::{Buffer, BufferCreateInfo, BufferUsage};
use vulkano::device::{Device, DeviceCreateInfo, QueueCreateInfo, QueueFlags};
use vulkano::instance::{Instance, InstanceCreateInfo};
use vulkano::memory::allocator::{AllocationCreateInfo, MemoryTypeFilter, StandardMemoryAllocator};
use vulkano::VulkanLibrary;

fn main() {
    create_window();
}

fn create_window() {
    // TODO: Need to move the device enumeration and creation out of this into a helper crate
    // Create an instance of Vulkano for mapping between Vulkano and the Vulkan library
    let library = VulkanLibrary::new().expect("no local Vulkan library/DLL");

    let instance =
        Instance::new(library, InstanceCreateInfo::default()).expect("failed to create instance");

    // Enum physical devices, could be dedicated gpu, integrated gpu, or software
    let physical_device = instance
        .enumerate_physical_devices()
        .expect("Could not enumerate devices")
        .next()
        .expect("No devices available");

    // Create a device representing the communication channel
    for family in physical_device.queue_family_properties() {
        println!(
            "Found a queue family with {:?} queue(s)",
            family.queue_count
        );
    }

    let queue_family_index = physical_device
        .queue_family_properties()
        .iter()
        .enumerate()
        .position(|(_queue_family_index, queue_family_properties)| {
            queue_family_properties
                .queue_flags
                .contains(QueueFlags::GRAPHICS)
        })
        .expect("couldn't find a graphical queue family") as u32;

    let (device, mut queues) = Device::new(
        physical_device,
        DeviceCreateInfo {
            // Pass the desired queue family to use by index
            queue_create_infos: vec![QueueCreateInfo {
                queue_family_index,
                ..Default::default()
            }],
            ..Default::default()
        },
    )
    .expect("Failed to create device");

    let queue = queues.next().unwrap();

    // Create a memory Allocator, make this ARC because you will need it in multiple places
    let memory_allocator = Arc::new(StandardMemoryAllocator::new_default(device.clone()));

    // Memory type filter and allocation
    let data: i32 = 12;
    let buffer = Buffer::from_data(
        memory_allocator.clone(),
        BufferCreateInfo {
            usage: BufferUsage::UNIFORM_BUFFER,
            ..Default::default()
        },
        AllocationCreateInfo {
            memory_type_filter: MemoryTypeFilter::PREFER_DEVICE
                | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
            ..Default::default()
        },
        data,
    )
    .expect("failed to create the buffer");
}
