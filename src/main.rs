use vulkano::VulkanLibrary;
use vulkano::instance::{Instance, InstanceCreateInfo};
use vulkano::device::{QueueFlags, Device, DeviceCreateInfo, QueueCreateInfo};

fn main() {
	
	let library = VulkanLibrary::new().expect("no local Vulkan library/DLL");
	let instance = Instance::new(library, InstanceCreateInfo::default())
		.expect("failed to create instance");
	let physical_device = instance
		.enumerate_physical_devices()
		.expect("could not enumerate devices")
		.next()
		.expect("no devices available");
	println!("ham");
	for family in physical_device.queue_family_properties() {
		println!("Found a queue family with {:?} queue(s)", family.queue_count);
	}
	let queue_family_index = physical_device
    .queue_family_properties()
    .iter()
    .enumerate()
    .position(|(_queue_family_index, queue_family_properties)| {
        queue_family_properties.queue_flags.contains(QueueFlags::GRAPHICS)
    })
    .expect("couldn't find a graphical queue family") as u32;
	println!("{queue_family_index}");
	let (device, mut queues) = Device::new(
		physical_device,
		DeviceCreateInfo {
			// here we pass the desired queue family to use by index
			queue_create_infos: vec![QueueCreateInfo {
			queue_family_index,
            ..Default::default()
        }],
		..Default::default()
		},
	)
	.expect("failed to create device");
	println!("tester9000");
	for q in queues {
		println!("{q:?}");
	}
	println!("{device:?}");


    /*println!("Hello, wordle!");
	println!("wow power");
	let mut loop_val: isize = 1;
	while loop_val <= 100 {
		print!("{loop_val}");
		loop_val *= 101;
		loop_val = loop_val % 13;
	}
	println!("build me")*/
	println!("gamed");
	println!("gamed2");
	println!("hamis");
}
