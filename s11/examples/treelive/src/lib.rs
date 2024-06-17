use std::mem;

type Tree = Option<Box<Node>>;

#[derive(Clone, Debug)]
pub struct IoTDevice {
    pub numerical_id: u64,
    pub address: String,
}

#[derive(Debug)]
struct Node {
    pub dev: IoTDevice,
    left: Tree,
    right: Tree,
}

#[derive(Debug)]
pub struct DeviceRegistry {
    root: Tree,
    pub length: u64,
}

impl IoTDevice {
    pub fn new(numerical_id: u64, address: String) -> IoTDevice {
        IoTDevice { numerical_id: numerical_id, address: address }
    }
}

impl Node {
    pub fn new(device: IoTDevice) -> Tree {
        Some(Box::new(Node {
            dev: device,
            left: None,
            right: None
        }))
    }
}

impl DeviceRegistry {

    pub fn new() -> DeviceRegistry {
        DeviceRegistry { root: None, length: 0 }
    }

    pub fn add(&mut self, device: IoTDevice) {
        self.length += 1;
        let root = mem::replace(&mut self.root, None);
        self.root = self.add_rec(root, device);
    }

    fn add_rec(&mut self, node: Tree, device: IoTDevice) -> Tree {
        match node {
            Some(mut n) => {
                if n.dev.numerical_id <= device.numerical_id {
                    n.left = self.add_rec(n.left, device);
                    Some(n)
                } else {
                    n.right = self.add_rec(n.right, device);
                    Some(n) }
            }
            _ => Node::new(device),
        }
    }

    pub fn find(&self, numerical_id: u64) -> Option<IoTDevice> {
       self.find_r(&self.root, numerical_id)
    }

    fn find_r(&self, node: &Tree, numerical_id: u64) -> Option<IoTDevice> {
       match node {
           Some(n) => {
               if n.dev.numerical_id == numerical_id {
                   Some(n.dev.clone())
               } else if n.dev.numerical_id < numerical_id {
                   self.find_r(&n.left, numerical_id)
               } else {
                   self.find_r(&n.right, numerical_id)
               }
            }
            _ => None,
       }
    }
}
