use heartree::{HeartRateTree}

fn main() {
    // Example usage of the HeartRateTree
    let mut rates = HeartRateTree::new();
    rates.insert(1625150000, 70);
    rates.insert(1625150540, 75);
    rates.insert(1625150600, 80);
    let current_time = 1625150600;
    println!("{}", tree);
    println!("Average heart rate in the last minute: {}", tree.average_last_minute(current_time));
}

