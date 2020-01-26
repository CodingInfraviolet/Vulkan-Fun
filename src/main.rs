use vulkano::instance::Instance;
use vulkano::instance::InstanceExtensions;
use vulkano::instance::PhysicalDevice;
use vulkano::device::Device;
use vulkano::device::DeviceExtensions;
use vulkano::device::Features;
use vulkano::buffer::BufferUsage;
use vulkano::buffer::CpuAccessibleBuffer;
use vulkano::command_buffer::AutoCommandBufferBuilder;
use vulkano_win::VkSurfaceBuild;
use winit::EventsLoop;
use winit::WindowBuilder;
use winit::dpi::LogicalSize;
use vulkano::swapchain::{self, Swapchain, SurfaceTransform, PresentMode};


fn main() {
    let instance = {
        let extensions = vulkano_win::required_extensions();
        Instance::new(None, &extensions, None).expect("failed to create Vulkan instance")
    };

    let devices = PhysicalDevice::enumerate(&instance);

    println!("Found devices:");
    for device in devices {
        println!("{}", device.name());
    }

    let physical_device = PhysicalDevice::enumerate(&instance).next().expect("Could not select device");

    let queue_family = physical_device.queue_families()
                             .find(|&q| q.supports_graphics())
                             .expect("couldn't find a graphical queue family");

    let device_ext = vulkano::device::DeviceExtensions {
        khr_swapchain: true,
        .. vulkano::device::DeviceExtensions::none()
    };

    let (device, mut queues) = {
        Device::new(physical_device, &physical_device.supported_features(), &device_ext,
                    [(queue_family, 0.5)].iter().cloned()).expect("failed to create device")
    };
    
    let queue = queues.next().unwrap();

    let data = 12;
    let buffer = CpuAccessibleBuffer::from_data(device.clone(), BufferUsage::all(), data).expect("failed to create buffer");

    let mut events_loop = EventsLoop::new();
    let surface = WindowBuilder::new().with_min_dimensions(LogicalSize::new(400.0, 200.0))
                                      .with_dimensions(LogicalSize::new(1280.0, 1024.0))
                                      .with_title("Vulkan Fun! :D")
                                      .with_decorations(true)
                                      .build_vk_surface(&events_loop, instance.clone()).unwrap();

    let caps = surface.capabilities(physical_device).expect("failed to get surface capabilities");

    let dimensions = caps.current_extent.unwrap_or([1280, 1024]);
    let alpha = caps.supported_composite_alpha.iter().next().unwrap();
    let format = caps.supported_formats[0].0;


    let (swapchain, images) = Swapchain::new(device.clone(), surface.clone(),
        caps.min_image_count, format, dimensions, 1, caps.supported_usage_flags, &queue,
        SurfaceTransform::Identity, alpha, PresentMode::Fifo, true, None)
        .expect("failed to create swapchain");

    events_loop.run_forever(|event| {
        match event {
            winit::Event::WindowEvent { event: winit::WindowEvent::CloseRequested, .. } => {
                winit::ControlFlow::Break
            },
            winit::Event::WindowEvent { event: winit::WindowEvent::CursorEntered {..}, ..} => {
                let (image_num, acquire_future) = swapchain::acquire_next_image(swapchain.clone(), None).unwrap();
                println!("Acquired swapchain {}", image_num);
                let clear_values = vec!(0.0, 0.0, 1.0, 1.0);
                let command_buffer = AutoCommandBufferBuilder::primary_one_time_submit(device.clone(), queue.family()).unwrap()
                    .build().unwrap();
                winit::ControlFlow::Continue
            }
            _ => winit::ControlFlow::Continue,
        }
    });
}
