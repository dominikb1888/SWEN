fn print_greeting(greeting: &str) {
    println!("{}", greeting);
}

fn main() {
    let my_string = "Hello, Rust!";
    print_greeting(my_string);
    println!("{}", my_string);
}
