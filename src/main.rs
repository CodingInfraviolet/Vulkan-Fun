use vulkano::instance::Instance;
use vulkano::instance::InstanceExtensions;
use vulkano::instance::PhysicalDevice;
use vulkano::device::Device;
use vulkano::device::DeviceExtensions;
use vulkano::device::Features;
use vulkano::buffer::BufferUsage;
use vulkano::buffer::CpuAccessibleBuffer;
use vulkano_win::VkSurfaceBuild;
use winit::EventsLoop;
use winit::WindowBuilder;
use winit::dpi::LogicalSize;

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

    let device = PhysicalDevice::enumerate(&instance).next().expect("Could not select device");

    let queue_family = device.queue_families()
                             .find(|&q| q.supports_graphics())
                             .expect("couldn't find a graphical queue family");

    let (device, mut queues) = {
    Device::new(device, &Features::none(), &DeviceExtensions::none(),
                [(queue_family, 0.5)].iter().cloned()).expect("failed to create device")
    };

    let data = 12;
    let buffer = CpuAccessibleBuffer::from_data(device.clone(), BufferUsage::all(), data).expect("failed to create buffer");

    let mut events_loop = EventsLoop::new();
    let surface = WindowBuilder::new().with_min_dimensions(LogicalSize::new(400.0, 200.0))
                                      .with_title("Vulkan Fun! :D")
                                      .with_decorations(true)
                                      .build_vk_surface(&events_loop, instance.clone()).unwrap();

    events_loop.run_forever(|event| {
        match event {
            winit::Event::WindowEvent { event: winit::WindowEvent::CloseRequested, .. } => {
                winit::ControlFlow::Break
            },
            _ => winit::ControlFlow::Continue,
        }
    });
}
