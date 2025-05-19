use std::fmt;

#[derive(Debug)]
struct BinarySearchTree {
    root: Option<Box<Node>>,
}

#[derive(Debug)]
struct Node {
    val: f64,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(val: f64) -> Self {
        Node {
            val,
            left: None,
            right: None,
        }
    }

    fn inorder(&self) -> Vec<f64> {
        let mut result = Vec::new();
        if let Some(left) = &self.left {
            result.extend(left.inorder());
        }
        result.push(self.val);
        if let Some(right) = &self.right {
            result.extend(right.inorder());
        }
        result
    }

    fn display(&self, indent: usize) {
        if let Some(right) = &self.right {
            right.display(indent + 4);
        }

        println!("{:indent$}{}", "", self.val, indent = indent);

        if let Some(left) = &self.left {
            left.display(indent + 4);
        }
    }
}

impl BinarySearchTree {
    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    fn from_sorted_slice(sorted: &[f64]) -> Option<Box<Node>> {
        if sorted.is_empty() {
            return None;
        }
        let mid = sorted.len() / 2;
        Some(Box::new(Node {
            val: sorted[mid],
            left: Self::from_sorted_slice(&sorted[..mid]),
            right: Self::from_sorted_slice(&sorted[mid + 1..]),
        }))
    }

    fn build_balanced(&mut self, values: &mut Vec<f64>) {
        values.sort_by(|a, b| a.partial_cmp(b).unwrap());
        self.root = Self::from_sorted_slice(values);
    }

    fn inorder(&self) -> Vec<f64> {
        match &self.root {
            Some(root) => root.inorder(),
            None => Vec::new(),
        }
    }

    fn display(&self) {
        match &self.root {
            Some(root) => root.display(0),
            None => println!("(empty tree)"),
        }
    }
}

impl fmt::Display for BinarySearchTree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BinarySearchTree({:?})", self.root)
    }
}

fn main() {
    use std::io::{self, Write};

    print!("Enter a list of numbers: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let mut nums: Vec<f64> = input
        .split_whitespace()
        .filter_map(|x| x.parse::<f64>().ok())
        .collect();

    let mut tree = BinarySearchTree::new();
    tree.build_balanced(&mut nums);

    println!("\nIn-order traversal (sorted):");
    for val in tree.inorder() {
        println!("{}", val);
    }

    println!("\nBalanced tree structure:");
    tree.display();
}

