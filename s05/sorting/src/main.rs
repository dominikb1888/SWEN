fn order<T: PartialOrd + std::fmt::Debug + Clone>(data: &mut [T]) -> &[T] {
    for i in 0..data.len() {
        for j in 0..data.len() {
            if data[i] < data[j] {
                let temp = &data[i].clone();
                data[i] = data[j].clone();
                data[j] = temp.clone();
            } else {
                continue
            }
        println!("{:?}", data);
        }
    }
    data
}

fn main() {
    let mut arr = [10, 30, 15, 20, 12];
    println!("{:?}", order(&mut arr));
}

//1st Attempt

// data |  comp        | result
// 10      10 > 30        [10]
// 30      30 > 15        [10, 15, 30]
// 20      20 > 12        [10, 15, 30, 12, 20]
//
// 2nd Attempt
//
// data | comp
// 10   | 10 > 10, 10 > 30, 10 > 15, 10 > 20, 10 > 12 |
// 30   | 30 > 10, 30 > 30, 30 > 15, 30 > 20, 30 > 12 |
//
//  result [10, 30, 0, 0, 0]
//  result [10, 15, 30, 0, 0]
//  result [10, 15, 20, 30, 0]
//  result [10, 12, 15, 20, 30]
