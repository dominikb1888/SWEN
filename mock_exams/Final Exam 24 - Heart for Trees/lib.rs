use std::fmt;
use chrono::{DateTime, Utc};

// Define the TreeNode struct
struct TreeNode {

}

impl TreeNode {
    fn new(timestamp: u64) -> Self {
        todo!();   // Implementation goes here
    }
}

// Define the HeartRateTree struct
struct HeartRateTree {
    root: Option<Box<TreeNode>>,
}

impl HeartRateTree {
    fn new() -> Self {
        todo!(); // Implementation goes here
    }

    fn insert(&mut self, timestamp: u64, value: u32) {
        todo!(); // Implementation goes here
    }

    fn insert_node(current: &mut Box<TreeNode>, new_node: Box<TreeNode>) {
        todo!(); // Implementation goes here
    }

    fn average_last_minute(&self, current_time: u64) -> f32 {
        todo!(); // Implementation goes here
    }

    fn sum_and_count(node: Option<&Box<TreeNode>>, start: u64, end: u64) -> (u32, u32) {
        todo!(); // Implementation goes here
    }

    // In-order traversal to collect nodes (Needed for tests, do not delete!)
    fn inorder_traversal(&self, node: &Option<Box<TreeNode>>, result: &mut Vec<(u64, u32)>) {
        if let Some(ref n) = node {
            self.inorder_traversal(&n.left, result);
            result.push((n.timestamp, n.heart_rate));
            self.inorder_traversal(&n.right, result);
        }
    }
}

// Implementing Display trait for TreeNode
impl fmt::Display for TreeNode {
    // Your implementation here
    }

// Implementing Display trait for HeartRateTree
impl fmt::Display for HeartRateTree {
    // Your implementation here
 }

impl HeartRateTree {
    fn display_node(&self, node: &Option<Box<TreeNode>>, f: &mut fmt::Formatter) -> fmt::Result {
    todo!(); // Your implementation here
    }
}
