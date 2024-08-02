use vulkano::format::Format;
use vulkano::image::{Image, ImageCreateInfo, ImageType, ImageUsage};

fn create_image() {
    // TODO Return image from here
    // Create an image with a 2D type, RGBA format, and transfer source usage
    let image = Image::new(
        memory_allocator.clone(),
        ImageCreateInfo {
            image_type: ImageType::Dim2d,
            format: Format::R8G8B8A8_UNORM,
            extent: [1024, 1024, 1],
            usage: ImageUsage::TRANSFER_DST | ImageUsage::TRANSFER_SRC,
            ..Default::default()
        },
        AllocationCreateInfo {
            memory_type_filter: MemoryTypeFilter::PREFER_DEVICE,
            ..Default::default()
        },
    )
    .unwrap();
}
