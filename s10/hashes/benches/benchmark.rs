use criterion::{criterion_group, criterion_main, Criterion};
use rand::{thread_rng, Rng};
use rand::seq::SliceRandom;
use std::cell::RefCell;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::iter;

use hashes::LocationInformation;
use hashes::adler32;
use hashes::hashcode;
use hashes::map;

thread_local!(static TEST_DATA: RefCell<Vec<(String, LocationInformation)>> = RefCell::new(random_location_info(10_000)));

fn new_location_info(name: &str) -> LocationInformation {
    LocationInformation {
        name: name.to_owned(),
        opened: "2018".to_owned(),
        address: "1 Curved Way, 10223 Someplace".to_owned(),
        security_group_name: format!("{}-sec", name),
    }
}

fn random_location_info(n: usize) -> Vec<(String, LocationInformation)> {
    let s = (b'A'..b'Z').map(|a| a as char).collect::<Vec<char>>();
    let mut rng = thread_rng();

    iter::repeat_with(|| s.clone())
        .take(n)
        .map(|mut s| {
            s.shuffle(&mut rng);
            let length = rng.gen_range(10..s.len());
            let k = s.into_iter().take(length).collect::<String>();
            let val = new_location_info(&k);
            (k, val)
        })
        .collect::<Vec<(String, LocationInformation)>>()
}

fn bench_hash_map_hashcode(c: &mut Criterion) {
    TEST_DATA.with(|location_info_cell| {
        let n = 10_000;
        let mut m = map::LocationCache::new(Box::new(|e: &String| hashcode(e.as_bytes()) as usize), n);
        let location_info = location_info_cell.borrow();
        for (key, value) in location_info.iter() {
            m.insert(key.clone(), value.clone());
        }

        assert_eq!(m.length, location_info.len());
        let mut rng = thread_rng();
        c.bench_function("bench_hash_map_hashcode", |b| {
            b.iter(|| {
                let pair = location_info.choose(&mut rng).expect("Nothing to choose from");
                m.get(&pair.0).expect("Not Found");
            });
        });
    });
}

fn bench_hash_map_adler32(c: &mut Criterion) {
    TEST_DATA.with(|location_info_cell| {
        let n = 10_000;
        let mut m = map::LocationCache::new(Box::new(|e: &String| adler32(e.as_bytes()) as usize), n);
        let location_info = location_info_cell.borrow();
        for (key, value) in location_info.iter() {
            m.insert(key.clone(), value.clone());
        }

        assert_eq!(m.length, location_info.len());
        let mut rng = thread_rng();
        c.bench_function("bench_hash_map_adler32", |b| {
            b.iter(|| {
                let pair = location_info.choose(&mut rng).expect("Nothing to choose from");
                m.get(&pair.0).expect("Not Found");
            });
        });
    });
}

fn bench_hash_map_defaulthasher(c: &mut Criterion) {
    TEST_DATA.with(|location_info_cell| {
        let n = 10_000;
        let mut m = map::LocationCache::new(
            Box::new(|e: &String| {
                let mut h = DefaultHasher::new();
                e.hash(&mut h);
                h.finish() as usize
            }),
            n,
        );
        let location_info = location_info_cell.borrow();
        for (key, value) in location_info.iter() {
            m.insert(key.clone(), value.clone());
        }

        assert_eq!(m.length, location_info.len());
        let mut rng = thread_rng();
        c.bench_function("bench_hash_map_defaulthasher", |b| {
            b.iter(|| {
                let pair = location_info.choose(&mut rng).expect("Nothing to choose from");
                m.get(&pair.0).expect("Not Found");
            });
        });
    });
}

fn bench_std_hash_map(c: &mut Criterion) {
    TEST_DATA.with(|location_info_cell| {
        let n = 10_000;
        let mut m = std::collections::HashMap::with_capacity(n);
        let location_info = location_info_cell.borrow();
        for (key, value) in location_info.iter() {
            m.insert(key.clone(), value.clone());
        }

        assert_eq!(m.len(), location_info.len());
        let mut rng = thread_rng();
        c.bench_function("bench_std_hash_map", |b| {
            b.iter(|| {
                let pair = location_info.choose(&mut rng).expect("Nothing to choose from");
                m.get(&pair.0).expect("Not Found");
            });
        });
    });
}

criterion_group!(
    benches,
    bench_hash_map_hashcode,
    bench_hash_map_adler32,
    bench_hash_map_defaulthasher,
    bench_std_hash_map
);
criterion_main!(benches);

