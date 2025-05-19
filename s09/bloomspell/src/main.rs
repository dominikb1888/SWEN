use ahash::{AHasher, RandomState};
use bitvec::prelude::*;
use std::hash::{Hash, Hasher};
use std::hash::BuildHasher;

const BLOOM_FILTER_SIZE: usize = 10_000; // bits
const NUM_HASH_FUNCTIONS: usize = 3;

struct BloomFilter {
    bits: BitVec,
    hashers: Vec<RandomState>,
}

impl BloomFilter {
    fn new() -> Self {
        let bits = bitvec![0; BLOOM_FILTER_SIZE];
        let hashers = (0..NUM_HASH_FUNCTIONS).map(|_| RandomState::new()).collect();
        Self { bits, hashers }
    }

    fn insert<T: Hash>(&mut self, item: &T) {
        for hasher in &self.hashers {
            let mut h = hasher.build_hasher();
            item.hash(&mut h);
            let index = (h.finish() as usize) % BLOOM_FILTER_SIZE;
            self.bits.set(index, true);
        }
    }

    fn contains<T: Hash>(&self, item: &T) -> bool {
        self.hashers.iter().all(|hasher| {
            let mut h = hasher.build_hasher();
            item.hash(&mut h);
            let index = (h.finish() as usize) % BLOOM_FILTER_SIZE;
            self.bits[index]
        })
    }
}

// Example spell checker
fn main() {
    let dictionary = vec![
        "apple", "banana", "grape", "orange", "strawberry", "watermelon",
    ];

    let mut bloom = BloomFilter::new();

    for word in &dictionary {
        bloom.insert(word);
    }

    let test_words = vec!["apple", "blueberry", "orange", "carrot", "banana"];

    for word in test_words {
        if bloom.contains(&word) {
            println!("'{}' might be in the dictionary.", word);
        } else {
            println!("'{}' is definitely not in the dictionary.", word);
        }
    }
}

