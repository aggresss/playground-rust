fn main() {
    let x = 2;

    match x {
        e @ 1..=5 => println!("got a range element {}", e),
        _ => println!("anything"),
    }
}
