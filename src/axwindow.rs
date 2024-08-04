use std::sync::Arc;

use vulkano::{
    buffer::{Buffer, BufferContents, BufferCreateInfo, BufferUsage},
    command_buffer::{
        allocator::StandardCommandBufferAllocator, AutoCommandBufferBuilder, RenderPassBeginInfo,
        SubpassBeginInfo, SubpassContents,
    },
    device::{
        physical::PhysicalDeviceType, Device, DeviceCreateInfo, DeviceExtensions, QueueCreateInfo,
        QueueFlags,
    },
    image::{view::ImageView, ImageUsage},
    instance::{Instance, InstanceCreateFlags, InstanceCreateInfo},
    library::VulkanLibrary,
    memory::allocator::{AllocationCreateInfo, MemoryTypeFilter, StandardMemoryAllocator},
    pipeline::{
        graphics::{
            color_blend::{ColorBlendAttachmentState, ColorBlendState},
            input_assembly::InputAssemblyState,
            multisample::MultisampleState,
            rasterization::RasterizationState,
            vertex_input::{Vertex, VertexDefinition},
            viewport::{Viewport, ViewportState},
            GraphicsPipelineCreateInfo,
        },
        layout::PipelineDescriptorSetLayoutCreateInfo,
        DynamicState, GraphicsPipeline, PipelineLayout, PipelineShaderStageCreateInfo,
    },
    render_pass::{Framebuffer, FramebufferCreateInfo, Subpass},
    swapchain::{
        acquire_next_image, Surface, Swapchain, SwapchainCreateInfo, SwapchainPresentInfo,
    },
    sync::{self, GpuFuture},
    Validated, VulkanError,
};

use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

pub(crate) fn create_window() {
    /*
    * This code snippet is setting up the Vulkan instance and required extensions for creating a
    * window using the `winit` library. Here's a breakdown of what each step does:
    */
    let event_loop = EventLoop::new();
    let library = VulkanLibrary::new().expect("no local Vulkan library/DLL");

    // Enable all non core extensions required to draw a window
    let required_extensions = Surface::required_extensions(&event_loop);

    let instance = Instance::new(
        library,
        InstanceCreateInfo {
            enabled_extensions: required_extensions,
            ..Default::default()
        },
    )
    .expect("failed to create instance");

    /*
     *  Create the actual window
     *  This code snippet is creating a window using the `winit` library, creating a surface from that
     *  window using Vulkan, and setting up device extensions required for Vulkan operations.
     */
    let window = Arc::new(WindowBuilder::new().build(&event_loop).unwrap());

    let surface = Surface::from_window(instance.clone(), window.clone()).unwrap();

    let device_extensions = DeviceExtensions {
        khr_swapchain: true,
        ..DeviceExtensions::empty()
    };

    /*
    *  Setup the physical device (CPU or GPU)
    *  This code snippet is responsible for finding a suitable physical device and queue family index
    * for rendering graphics using Vulkan. Here's a breakdown of what it does:
    */
    let (physical_device, queue_family_index) = instance
        .enumerate_physical_devices()
        .unwrap()
        .filter(|p| p.supported_extensions().contains(&device_extensions))
        .filter_map(|p| {
            p.queue_family_properties()
                .iter()
                .enumerate()
                .position(|(i, q)| {
                    q.queue_flags.intersects(QueueFlags::GRAPHICS)
                        && p.surface_support(i as u32, &surface).unwrap_or(false)
                })
                .map(|i| (p, i as u32))
        })
        .min_by_key(|(p, _)| match p.properties().device_type {
            PhysicalDeviceType::DiscreteGpu => 0,
            PhysicalDeviceType::IntegratedGpu => 1,
            PhysicalDeviceType::VirtualGpu => 2,
            PhysicalDeviceType::Cpu => 3,
            PhysicalDeviceType::Other => 4,
            _ => 5,
        })
        .expect("No suitable physical device found");

    /* Event Loop, this blocks the main thread forever
    * This code snippet is setting up an event loop that listens for window events. When a
    * `WindowEvent::CloseRequested` event is triggered, it sets the `control_flow` variable to
    * `ControlFlow::Exit`, which will cause the event loop to exit, effectively closing the window and
    * stopping the program.
    */
    event_loop.run(|event, _, control_flow| match event {
        Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } => {
            *control_flow = ControlFlow::Exit;
        }
        _ => (),
    });
}
