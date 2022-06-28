use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let _f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("{:?}", error);
            })
        } else {
            panic!("{:?}", error);
        }
    });
}
