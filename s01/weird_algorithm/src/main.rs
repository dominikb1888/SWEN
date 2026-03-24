fn main() {
    let input = std::io::read_to_string(std::io::stdin()).unwrap();
    let mut n = input.trim().parse::<u64>().expect("Not a valid number as input");
    print!("{}", n);
    loop {
        if n == 1 { break; }
        n = if n % 2 == 0 { n / 2 } else { n * 3 + 1 };
        print!(" {n}");
    }
}
