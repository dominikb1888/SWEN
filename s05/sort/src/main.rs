use std::time::Instant;
use rand::seq::SliceRandom;
use rand::thread_rng;

fn main() {
    let mut rng = thread_rng();

    let small = vec![64, 25, 12, 22, 11, 45, 78, 1, 99, 3, 56, 42, 10];
    let mut medium: Vec<u32> = (0..1000).collect();
    let mut large: Vec<u32> = (0..10_000).collect();
    medium.shuffle(&mut rng);
    large.shuffle(&mut rng);

    println!("===== Small Vec (len = {}) =====", small.len());
    run_all_sorts(&small);

    println!("\n===== Medium Vec (len = {}) =====", medium.len());
    run_all_sorts(&medium);

    println!("\n===== Large Vec (len = {}) =====", large.len());
    run_all_sorts(&large);
}

fn run_all_sorts<T: Ord + Clone + Copy>(data: &[T]) {
    let mut selection = data.to_vec();
    time_sort("Selection Sort", &mut selection, selection_sort);

    let mut merge = data.to_vec();
    time_sort("Merge Sort", &mut merge, merge_sort);

    let mut quick = data.to_vec();
    time_sort("Quick Sort", &mut quick, quick_sort);
}

fn time_sort<T: Ord + Clone + Copy, F>(label: &str, data: &mut [T], sort_fn: F)
where
    F: Fn(&mut [T]),
{
    let start = Instant::now();
    sort_fn(data);
    let duration = start.elapsed();
    println!("{:<15} took {:?}", label, duration);
}

// ---------- Selection Sort ----------
pub fn selection_sort<T: Ord>(arr: &mut [T]) {
    let len = arr.len();
    for i in 0..len {
        let mut min_index = i;
        for j in (i + 1)..len {
            if arr[j] < arr[min_index] {
                min_index = j;
            }
        }
        arr.swap(i, min_index);
    }
}

// ---------- Merge Sort ----------
pub fn merge_sort<T: Ord + Clone>(arr: &mut [T]) {
    let len = arr.len();
    if len <= 1 {
        return;
    }

    let mid = len / 2;
    let mut left = arr[..mid].to_vec();
    let mut right = arr[mid..].to_vec();

    merge_sort(&mut left);
    merge_sort(&mut right);

    merge(arr, &left, &right);
}

fn merge<T: Ord + Clone>(arr: &mut [T], left: &[T], right: &[T]) {
    let mut i = 0;
    let mut j = 0;
    let mut k = 0;

    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            arr[k] = left[i].clone();
            i += 1;
        } else {
            arr[k] = right[j].clone();
            j += 1;
        }
        k += 1;
    }

    while i < left.len() {
        arr[k] = left[i].clone();
        i += 1;
        k += 1;
    }

    while j < right.len() {
        arr[k] = right[j].clone();
        j += 1;
        k += 1;
    }
}

// ---------- Quick Sort ----------
pub fn quick_sort<T: Ord + Clone>(arr: &mut [T]) {
    let len = arr.len();
    if len <= 1 {
        return;
    }

    let pivot_index = partition(arr);
    let (left, right) = arr.split_at_mut(pivot_index);
    quick_sort(left);
    quick_sort(&mut right[1..]); // exclude pivot
}

fn partition<T: Ord + Clone>(arr: &mut [T]) -> usize {
    let len = arr.len();
    let pivot_index = len - 1;
    let pivot = &arr[pivot_index].to_owned();

    let mut i = 0;
    for j in 0..pivot_index {
        if &arr[j] <= pivot {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, pivot_index);
    i
}

