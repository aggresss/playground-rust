use std::fs::File;
use std::io;
use std::io::Read;

fn read_username_frome_file() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

fn main() {
    let _ = read_username_frome_file();
}
