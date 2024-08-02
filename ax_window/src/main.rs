use std::sync::Arc;

use vulkano::instance::{Instance, InstanceCreateInfo};
use vulkano::library::VulkanLibrary;
use vulkano::swapchain::Surface;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

fn main() {
    create_window();
}

fn create_window() {
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

    // Create the actual window
    let window = Arc::new(WindowBuilder::new().build(&event_loop).unwrap());

    // Event Loop, this blocks the main thread forever
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
