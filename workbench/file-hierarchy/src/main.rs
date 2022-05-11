mod foo;

fn print_alpha() {
    println!("alpha");
}

fn main() {
    print_alpha();
    foo::bar::print_beta();
    foo::bar::sigma::print_gamma();
}
