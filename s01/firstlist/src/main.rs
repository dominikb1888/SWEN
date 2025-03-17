use std::mem;

type Link = Option<Box<Node>>;

#[derive(Debug)]
pub struct List {
    head: Link,
    tail: Link,
    size: usize
}

#[derive(Debug)]
struct Node {
    elem: i64,
    next: Link,
}

impl List {
    pub fn new() -> Self {
        List {
            head: None,
            tail: None,
            size: 0,
        }
    }

    pub fn push(&mut self, elem: i64) {
        if self.size == 0 {
            self.tail = self.head.take(); // TODO: Fix borrowing issue here!
        }
        let new_node = Box::new(Node {
            elem: elem,
            next: mem::replace(&mut self.head, None),
        });
        self.size += 1;
        self.head = Some(new_node);
    }

    pub fn peek(&mut self) -> Option<i64> {
         self.head.as_ref().map(|node| {
            &node.elem
        }).copied()
    }
}

fn main() {
    let mut new_list = List::new();
    new_list.push(2);
    new_list.push(1);
    println!("{:?}", new_list);
}
