use std::mem;


#[derive(Debug)]
pub struct List {
    head: Link,
}

#[derive(Debug)]
enum Link {
    Empty,
    More(Box<Node>),
}

#[derive(Debug)]
struct Node {
    elem: i32,
    next: Link,
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    pub fn push(&mut self, elem: i32) {
        
        let new_node = Box::new( Node {
            elem: elem,
            next: mem::replace(&mut self.head, Link::Empty)
        });

        self.head = Link::More(new_node);
    }
}

// TODO: 
// - introduce TDD and write unit tests
// - impl pop
// - impl Drop Trait for List
// - Replace mem-replace with take()
// - Replace match with map()
