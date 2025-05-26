mod person;
mod heap;

use person::Person;
use heap::PersonHeap;

fn main() {
    let mut heap = PersonHeap::new();
    heap.insert(Person {
        name: "Alice".to_string(),
        age: 30,
        priority: 2,
    });
    heap.insert(Person {
        name: "Bob".to_string(),
        age: 25,
        priority: 5,
    });

    while let Some(person) = heap.delete() {
        println!("Removed: {:?}", person);
    }
}

