enum HttpStatus {
    Ok = 200,
    NotModified = 304,
    NotFound = 404,
}


fn main() {
    let n = 200;
    match n {
        200 => Some(HttpStatus::Ok),
        304 => Some(HttpStatus::NotModified),
        404 => Some(HttpStatus::NotFound),
        _ => None,
    };
}
