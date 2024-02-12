// Rust with unnecessary annotations
fn build_vector() -> Vec<i16> {
    let mut v: Vec<i16> = Vec::<i16>::new(); v.push(10i16);
    v.push(20i16);
    v
}

// Rust with effective annotation
fn build_vector() -> Vec<i16> {
    let mut v = Vec::new();
    v.push(10);
    v.push(20);
    v
}

// Python with annotation
def build_vector() -> [int]:
    v = []
    v.append(10)
    v.append(10)

    return v

