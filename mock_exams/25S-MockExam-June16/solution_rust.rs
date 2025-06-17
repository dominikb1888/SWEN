#[derive(Debug)]
struct Node<T> {
     data: T,
     next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
     fn new(data: T) -> Self {
         Node { data, next: None }
     }
}

#[derive(Debug)]
struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> LinkedList<T> {
    fn new() -> Self {
        LinkedList { head: None }
    }

    fn insert_at_end(&mut self, value: T) -> Option<&Box<Node<T>>> {
        let new_node = Box::new(Node::new(value));

        if self.head.is_none() {
            self.head = Some(new_node);
            return self.head.as_ref()
        }

        let mut current_node = self.head.as_mut().unwrap(); // Get mutable reference to the head
        while let Some(ref mut next_node) = current_node.next {
            current_node = next_node; // Move to the next node
        }
        current_node.next = Some(new_node); // Attach new node at the end
        self.head.as_ref() // Return the original head
    }
}

fn main() {
    let mut ll = LinkedList::new();
    ll.insert_at_end(1);
    ll.insert_at_end(10);
    println!("{:?}", ll);
}
