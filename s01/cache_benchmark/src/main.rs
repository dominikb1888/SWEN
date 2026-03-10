use std::collections::LinkedList;

const SIZE: usize = 1_000_000;

// --- Arena Implementation ---
struct Node<T> { data: T, next: Option<usize> }
struct CacheFriendlyList<T> { arena: Vec<Node<T>>, head: Option<usize> }

impl<T> CacheFriendlyList<T> {
    fn new() -> Self {
        CacheFriendlyList { arena: Vec::with_capacity(SIZE), head: None }
    }
    fn push_front(&mut self, data: T) {
        let new_index = self.arena.len();
        self.arena.push(Node { data, next: self.head });
        self.head = Some(new_index);
    }
}

fn main() {
    // 1. Setup Phase (Using `i % 7` to scramble the data and defeat the optimizer)
    let vec_data: Vec<i32> = (0..SIZE as i32).map(|i| i % 7).collect();
    
    let mut arena_data = CacheFriendlyList::new();
    for i in (0..SIZE as i32).rev() { 
        arena_data.push_front(i % 7); 
    }

    let mut list_data: LinkedList<i32> = LinkedList::new();
    for i in 0..SIZE as i32 { 
        list_data.push_back(i % 7); 
    }

    // 2. Execution Phase
    let sum1: i32 = vec_data.iter().sum();
    
    let mut sum2 = 0;
    let mut current = arena_data.head;
    while let Some(idx) = current {
        sum2 += arena_data.arena[idx].data;
        current = arena_data.arena[idx].next;
    }

    let sum3: i32 = list_data.iter().sum();

    // 3. The Ultimate Anti-Optimizer
    // Printing to the console physically forces the CPU to execute the math
    println!("Results: {}, {}, {}", sum1, sum2, sum3);
}
