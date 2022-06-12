fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}

fn main() {
    let x: (u32, u16) = (1, 2);
    print_type_of(&x);
    print_type_of(&{});
    print_type_of(&());
}
