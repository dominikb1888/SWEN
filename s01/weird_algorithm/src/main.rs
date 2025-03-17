/// DO NOT CHANGE FUNCTION NAME AND INPUT,
/// DO ADD OUTPUT TYPE
fn process_weird_algorithm(input: &str) -> String {
    let mut input_int = input.trim().parse::<i64>().expect("");
    let mut result: Vec<i64> = vec![input_int];
    if input_int != 1 {
        loop {
            input_int = if input_int % 2 == 0 { input_int / 2 } else { input_int * 3  + 1 };
            result.push(input_int);
            if input_int == 1 {
                break;
            }
        }
    }

    result.iter().map(|i| i.to_string()).collect::<Vec<String>>().join(" ")
}

///
/// DO CHANGE CODE BELOW THIS LINE
///
// src/lib.rs or src/main.rs
pub fn process(input: &str) -> String {
    process_weird_algorithm(&input)
}

fn main() {
    // Read from stdin and write to stdout
    let input = std::io::read_to_string(std::io::stdin()).unwrap();
    let output = process(&input);
    println!("{}", output);
}

#[cfg(test)]
mod tests {
    use super::*;

    // Include the dynamically generated test code
    include!(concat!(env!("OUT_DIR"), "/generated_tests.rs"));
}
