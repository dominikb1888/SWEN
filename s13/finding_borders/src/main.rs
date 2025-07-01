use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut line = String::new();
    stdin.lock().read_line(&mut line).unwrap();

    let s = line.trim_end().as_bytes();
    let n = s.len();
    for i in 1..n {
        if &s[0..i] == &s[n - i..] {
            print!("{} ", i);
        }
    }
}

