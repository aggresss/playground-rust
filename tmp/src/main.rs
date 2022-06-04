struct Color(u8, u8, u8);

fn main() {
    let purple = Color(128, 0, 128);
    let Color(r, g, b) = purple;
    println!("Purple = rgb({},{},{})", r, g, b);
}
