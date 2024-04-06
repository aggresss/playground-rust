// Subtraits & Supertraits

trait Supertraits {
    fn method(&self) {
        println!("in supertrait");
    }
}

trait Subtraits: Supertraits {
    fn method(&self) {
        println!("in subtrait");
    }
}

struct SomeType;

impl Supertraits for SomeType {}

impl Subtraits for SomeType {}

fn main() {
    // aubiguous method call
    // SomeType.method();
    let st = SomeType{};
    <SomeType as Supertraits>::method(&st);
    <SomeType as Subtraits>::method(&st);
}
