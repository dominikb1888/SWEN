use std::cmp::{max, Ordering};
use std::fmt;

#[derive(Debug)]
struct AVLTree {
    root: Option<Box<Node>>,
}

#[derive(Debug)]
struct Node {
    val: f64,
    height: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(val: f64) -> Self {
        Node {
            val,
            height: 1,
            left: None,
            right: None,
        }
    }

    fn height(node: &Option<Box<Node>>) -> i32 {
        node.as_ref().map_or(0, |n| n.height)
    }

    fn update_height(&mut self) {
        self.height = 1 + max(Self::height(&self.left), Self::height(&self.right));
    }

    fn balance_factor(&self) -> i32 {
        Self::height(&self.left) - Self::height(&self.right)
    }

    fn rotate_right(mut y: Box<Node>) -> Box<Node> {
        let mut x = y.left.take().expect("rotate_right: left child missing");
        y.left = x.right.take();
        y.update_height();
        x.right = Some(y);
        x.update_height();
        x
    }

    fn rotate_left(mut x: Box<Node>) -> Box<Node> {
        let mut y = x.right.take().expect("rotate_left: right child missing");
        x.right = y.left.take();
        x.update_height();
        y.left = Some(x);
        y.update_height();
        y
    }

    fn rebalance(mut node: Box<Node>) -> Box<Node> {
        node.update_height();
        let balance = node.balance_factor();

        if balance > 1 {
            if node.left.as_ref().unwrap().balance_factor() < 0 {
                node.left = Some(Self::rotate_left(node.left.take().unwrap()));
            }
            return Self::rotate_right(node);
        }

        if balance < -1 {
            if node.right.as_ref().unwrap().balance_factor() > 0 {
                node.right = Some(Self::rotate_right(node.right.take().unwrap()));
            }
            return Self::rotate_left(node);
        }

        node
    }

    fn insert(node: Option<Box<Node>>, val: f64) -> Box<Node> {
        match node {
            Some(mut n) => {
                match val.partial_cmp(&n.val).unwrap() {
                    Ordering::Less => n.left = Some(Self::insert(n.left.take(), val)),
                    Ordering::Greater => n.right = Some(Self::insert(n.right.take(), val)),
                    Ordering::Equal => return n, // Duplicate values are ignored
                }
                Self::rebalance(n)
            }
            None => Box::new(Node::new(val)),
        }
    }

    fn inorder(node: &Option<Box<Node>>, result: &mut Vec<f64>) {
        if let Some(n) = node {
            Self::inorder(&n.left, result);
            result.push(n.val);
            Self::inorder(&n.right, result);
        }
    }

    fn display(node: &Option<Box<Node>>, indent: usize) {
        if let Some(n) = node {
            Self::display(&n.right, indent + 4);
            println!("{:indent$}{}", "", n.val, indent = indent);
            Self::display(&n.left, indent + 4);
        }
    }
}

impl AVLTree {
    fn new() -> Self {
        AVLTree { root: None }
    }

    fn insert(&mut self, val: f64) {
        self.root = Some(Node::insert(self.root.take(), val));
    }

    fn inorder(&self) -> Vec<f64> {
        let mut result = Vec::new();
        Node::inorder(&self.root, &mut result);
        result
    }

    fn display(&self) {
        Node::display(&self.root, 0);
    }
}

impl fmt::Display for AVLTree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "AVLTree({:?})", self.root)
    }
}

fn main() {
    use std::io::{self, Write};

    print!("Enter a list of numbers: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let nums: Vec<f64> = input
        .split_whitespace()
        .filter_map(|x| x.parse::<f64>().ok())
        .collect();

    let mut tree = AVLTree::new();
    for val in nums {
        tree.insert(val);
    }

    println!("\nIn-order traversal (sorted):");
    for val in tree.inorder() {
        println!("{}", val);
    }

    println!("\nAVL Tree Structure:");
    tree.display();
}

