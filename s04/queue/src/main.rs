/// A first-in, first-out queue of characters.
#[derive(Debug, Clone)]
pub struct Queue<T> {
    older: Vec<T>, // older elements, eldest last.
    younger: Vec<T> // younger elements, youngest last.
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue { older: Vec::new(), younger: Vec::new() }
    }

    /// Push a character onto the back of a queue.
    pub fn push(&mut self, t: T) {
        self.younger.push(t);
    }

    /// Pop a character off the front of a queue. Return `Some(c)` if there /// was a character to pop, or `None` if the queue was empty.
    pub fn pop(&mut self) -> Option<T> {
        if self.older.is_empty() {
            if self.younger.is_empty() {
                return None;
            }
        // Bring the elements in younger over to older, and put them in // the promised order.
        use std::mem::swap;
        swap(&mut self.older, &mut self.younger);
            self.older.reverse();
        }
        // Now older is guaranteed to have something. Vec's pop method // already returns an Option, so we're set.
        self.older.pop()
    }
}

fn main() {
    let mut queue = Queue::<char>::new();
    queue.push('a'); // Queue: a
    println!("{:?}", &queue);
    queue.push('b'); // Queue: a, b
    println!("{:?}", &queue);
    queue.push('c'); // Queue: a, b, c
    println!("{:?}", &queue);
    queue.pop();     // Queue: b, c
    println!("{:?}", &queue);
    queue.push('d'); // Queue: a, b, c
    println!("{:?}", &queue);

    let mut number_queue = Queue::<u8>::new();
    number_queue.push(1);
    println!("{:?}", &number_queue);

    let clone_queue = number_queue.clone();
    println!("{:?}", &clone_queue);

}
