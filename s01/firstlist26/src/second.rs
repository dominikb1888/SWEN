use std::mem;


#[derive(Debug)]
pub struct List {
    head: Link,
}

type Link = Option<Box<Node>>;

#[derive(Debug)]
struct Node {
    elem: i32,
    next: Link,
}

impl List {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, elem: i32) {      
        let new_node = Box::new( Node {
            elem: elem,
            next: self.head.take()
        });

        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
       // match self.head.take() { // Link
       //      None => None,
       //      Some(node) => {
       //          self.head = node.next;
       //          Some(node.elem)
       //      }
       //  }
       // if let Some(node) = self.head.take() {
       //      self.head = node.next;
       //      Some(node.elem)
       // } else {
       //     None
       // }
       self.head.take().map(|node| {
         self.head = node.next;
         node.elem
       })

    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, None);
        // `while let` == "do this thing until this pattern doesn't match"
        while let Some(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, None);
            // boxed_node goes out of scope and gets dropped here;
            // but its Node's `next` field has been set to Link::Empty
            // so no unbounded recursion occurs.
        }
    }
}


#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn test_push_pop() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);

    }

    #[test]
    fn test_drop() {
        let mut list = List::new();

        list.push(6);
        list.push(7);
        list.push(8);

        assert_eq!(drop(list), ());
    }
}




// TODO: 
// - introduce TDD and write unit tests
// - impl pop
// - impl Drop Trait for List
// - Replace mem-replace with take()
// - Replace match with map()
