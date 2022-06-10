extern crate mylib;
use mylib::make_answer;

make_answer!();

fn main() {
    println!("{}", answer());
}