#[derive(Debug)]
enum HttpClass {
    Information,
    Success,
    Redirect,
    ClientError,
    ServerError,
}

fn classify_status_code(code: usize) -> Option<HttpClass> {
    match code {
        100 ..= 103 => Some(HttpClass::Information),
        200 ..= 208 | 226 => Some(HttpClass::Success),
        300 ..= 308 => Some(HttpClass::Redirect),
        400 ..= 418 | 421 ..= 429 | 431 | 451 => Some(HttpClass::ClientError),
        500 ..= 511 => Some(HttpClass::ServerError),
        _ => None,
    }
}


fn main() {
    println!("{:?}", classify_status_code(300));
    println!("{:?}", classify_status_code(431));
}
