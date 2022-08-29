fn main() {
    let some_number = Some(9);

    let another_number = some_number.map(|n| n-1).map(|n| n*n).and_then(|n| divide(n, 4));

    println!("{:?}", another_number.unwrap());
}

fn divide(number: i32, divisor: i32) -> Option<i32> {
    if divisor != 0 {
        Some(number/divisor)
    } else {
        None
    }
}
