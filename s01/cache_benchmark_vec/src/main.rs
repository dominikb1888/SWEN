use std::hint::black_box;

// 3 Million items! 
// Vec = 12 MB of Heap RAM
// Arena = 48 MB of Heap RAM
const SIZE: usize = 100_000_000;

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
    // 1. Setup Phase
    
    // Structure 1: The Massive Dynamic Vec (Heap)
    let vec_data: Vec<i32> = (0..SIZE as i32).map(|i| i % 7).collect();
    
    // Structure 2: The Massive Contiguous Arena List (Heap)
    let mut arena_data = CacheFriendlyList::new();
    for i in (0..SIZE as i32).rev() { 
        arena_data.push_front(i % 7); 
    }

    // 2. Execution Phase
    let sum1: i32 = vec_data.iter().sum();
    
    let mut sum2 = 0;
    let mut current = arena_data.head;
    while let Some(idx) = current {
        sum2 += arena_data.arena[idx].data;
        current = arena_data.arena[idx].next;
    }

    // 3. The Ultimate Anti-Optimizer
    println!("Results: {}, {}", sum1, sum2);
}
