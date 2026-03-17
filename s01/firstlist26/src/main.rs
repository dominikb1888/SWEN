use firstlist26::first::List;


fn main() {
    let mut list = List::new();
    println!("{:?}", list);
    list.push(32);
    println!("{:?}", list);
    list.push(40);
    println!("{:?}", list);
}
