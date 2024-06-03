use std::mem;

struct MyStruct {
       a: u8,
       b: u8,
       c: u8
}

fn main() {
       assert_eq!(mem::size_of::<MyStruct>(), 3 * mem::size_of::<u8>());
       assert_eq!(mem::size_of::<[MyStruct; 2]>(), 3 * mem::size_of::<u8>() * 2);

      println!("{}", mem::size_of::<MyStruct>());
      println!("{}", 3 * mem::size_of::<u8>());
}
