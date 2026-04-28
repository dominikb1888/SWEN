use firstlist26::second::List;
use std::time::Instant;

fn main() {
    let sizes = [10_000_000];
    
    println!("{:>10} | {:>15} | {:>15}", "Size", "vec_sort (ms)", "merge_sort (ms)");
    println!("{:-<10}-+-{:-<15}-+-{:-<15}", "", "", "");

    for &size in &sizes {
        // Benchmark vec_sort
        let mut list_vec = List::new();
        for i in 0..size {
            list_vec.push((i as u64 * 31) % size as u64);
        }
        
        let start = Instant::now();
        list_vec.vec_sort();
        let duration_vec = start.elapsed();
        drop(list_vec); // Free memory early

        // Benchmark merge_sort
        let mut list_merge = List::new();
        for i in 0..size {
            list_merge.push((i as u64 * 31) % size as u64);
        }
        
        let start = Instant::now();
        list_merge.merge_sort();
        let duration_merge = start.elapsed();
        drop(list_merge); // Free memory early

        println!("{:>10} | {:>15.2} | {:>15.2}", size, duration_vec.as_secs_f64() * 1000.0, duration_merge.as_secs_f64() * 1000.0);
    }
}
