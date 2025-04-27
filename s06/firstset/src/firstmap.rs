use crate::firstmap::FirstMap;

fn main() {
    let mut map: FirstMap<&str, i32> = FirstMap::new();
    map.insert("hello", 32);
    println!("{:?}", map.get(&"hello")); // Should print Some(32)
    map.insert("hello", 99);
    println!("{:?}", map.get(&"hello")); // Should print Some(99)
    map.remove("hello");
    println!("{:?}", map.get(&"hello")); // Should print None
}

mod firstmap {
    type Entry<K, V> = Vec<(K, V)>;

    pub struct FirstMap<K, V>
    where
        K: PartialEq + Clone + AsRef<[u8]>,
        V: Clone,
    {
        store: Box<[Entry<K, V>]>,
        pub length: usize,
    }

    impl<K, V> FirstMap<K, V>
    where
        K: PartialEq + Clone + AsRef<[u8]>,
        V: Clone,
    {
        pub fn new() -> Self {
            Self {
                store: vec![Vec::new(); 10].into_boxed_slice(),
                length: 0,
            }
        }

        pub fn hash_fn(bytes: &[u8]) -> usize {
            let mut a = 0_u32;
            for (i, b) in bytes.iter().enumerate() {
                a ^= *b as u32;
                a = a.wrapping_shl((i % 4) as u32);
            }
            a as usize
        }

        pub fn insert(&mut self, key: K, value: V) {
            let h = Self::hash_fn(key.as_ref());
            let idx = h & (self.store.len() - 1);
            match self.store[idx].iter().position(|e| e.0 == key) {
                Some(pos) => self.store[idx][pos] = (key, value),
                None => {
                    self.store[idx].push((key, value));
                    self.length += 1;
                }
            }
        }

        pub fn get(&self, key: &K) -> Option<V> {
            let h = Self::hash_fn(key.as_ref());
            let idx = h & (self.store.len() - 1);
            self.store[idx]
                .iter()
                .find(|e| &e.0 == key)
                .map(|e| e.1.clone())
        }

        pub fn remove(&mut self, key: K) -> Option<V> {
            let h = Self::hash_fn(key.as_ref());
            let idx = h & (self.store.len() - 1);
            match self.store[idx].iter().position(|e| e.0 == key) {
                Some(pos) => {
                    self.length -= 1;
                    Some(self.store[idx].remove(pos).1)
                }
                None => None,
            }
        }
    }
}

