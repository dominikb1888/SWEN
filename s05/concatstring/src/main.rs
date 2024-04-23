use std::ops::Mul;

// Define a newtype wrapper around String
struct ConcatString(String);

impl Mul<u32> for ConcatString {
    type Output = String;

    fn mul(self, rhs: u32) -> String {
        let ConcatString(s) = self;
        s.repeat(rhs as usize)
    }
}

fn main() {
    let concat_string = ConcatString("hello".to_string());
    let result = concat_string * 3;
    println!("{}", result);  // Output: hellohellohello
}
