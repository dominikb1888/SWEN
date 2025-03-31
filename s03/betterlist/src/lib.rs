use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt;
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
            next: self.head.take(),
        });
        self.size += 1;
        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<i64> {
        let result;
        match self.head.take() {
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

impl Display for List {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut cur_link = &self.head;
        let mut output = String::from("HEAD");

        while let Some(boxed_node) = cur_link {
            output.push_str(&format!(" -> {}", boxed_node.elem));
            cur_link = &boxed_node.next;
        }

        write!(f, "{} -> NONE", output)
    }
}
// HEAD -> 1 -> 2 -> 3 -> NONE

impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
        }
    }
}
