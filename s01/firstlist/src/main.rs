use std::mem;

type Link = Option<Box<Node>>;

#[derive(Debug)]
pub struct List {
    head: Link,
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
            size: 0,
        }
    }

    pub fn push(&mut self, elem: i64) {
        let new_node = Box::new(Node {
            elem: elem,
            next: mem::replace(&mut self.head, None),
        });
        self.size += 1;
        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<i64>{
        let result;
        match mem::replace(&mut self.head, None) {
            None =>  {
                result = None;
            },
            Some(node) => {
                 result = Some(node.elem);
                 self.head = node.next;
                 self.size -= 1;
            }
        }
        result
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
    new_list.pop();
    println!("{:?}", new_list);
}
