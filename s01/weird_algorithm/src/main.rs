fn main() {
    // Read from stdin and write to stdout
    let input = std::io::read_to_string(std::io::stdin()).unwrap();
    // Maximum n = 1_000_000
    let mut n = input.trim().parse::<u64>().expect("Not a valid number as input"); // Result<u32, io::Error>
    let mut numbers: Vec<u64> = vec![n]; // n = 2

    loop {
         // numbers == [2, 1]
        if n == 1 {
            break;
        }

        if n % 2 == 0 {
            // Add the result of n divided by two to our numbers vector    
            n = n / 2;
        } else {
            // Multiply by Three and add 1
            n = n * 3 + 1;
        }
        
        // n == 1 !!
        numbers.push(n);

    }
    
    //                   [2, 1] -> Iterable([2, 1])  -> Iterable(["2", "1"])  ->  Iterable("2 1") -> "2 1"
    let result: String = numbers // [2, 1]
        .into_iter()            // Iterator([2, 1])
        .map(|n| n.to_string()) // Map(["2", "1"]
        .collect::<Vec<String>>() // ["2", "1"]
        .join(" ");  // Iterator ("2 1")
    
    println!("{}", result)

}
