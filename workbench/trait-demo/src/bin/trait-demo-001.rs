trait Default {
    fn default() -> Self;
}

fn main() {
    let zero: i32 = Default::default();
    let zero = i32::default();
}
