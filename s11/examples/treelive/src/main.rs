use treelive::{DeviceRegistry, IoTDevice};

fn main() {
    let mut tree = DeviceRegistry::new();
    tree.add(IoTDevice::new(5, "home5".to_string()));
    tree.add(IoTDevice::new(4, "home4".to_string()));
    tree.add(IoTDevice::new(6, "home6".to_string()));
    tree.add(IoTDevice::new(3, "home3".to_string()));
    tree.add(IoTDevice::new(7, "home7".to_string()));
    tree.add(IoTDevice::new(2, "home2".to_string()));
    tree.add(IoTDevice::new(8, "home8".to_string()));

    println!("{:?}", tree);
    println!("{:?}", tree.find(4));

}
