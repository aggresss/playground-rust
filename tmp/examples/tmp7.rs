fn f1(_s: &String) -> &str {
    "alpha"
}

fn f2(_s: &str) -> &str {
    "beta"
}


fn main() {
    println!("{}", f1(&"f1".to_string()));
    println!("{}", f2("f2"));
}
