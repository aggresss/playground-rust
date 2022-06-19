use std::cell::Cell;

#[derive(Debug)]
struct Person {
    name: String,
    age: Cell<usize>,
}

fn main() {
    let person = Person { name: "Job Biden".to_string(), age: Cell::new(79) };
    let person_ref: &Person = &person;

    println!("Age is : {:?}", person_ref);
    person_ref.age.set(83);
    println!("Age is: {:?}", person_ref);
}
