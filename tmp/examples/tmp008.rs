// 01

pub trait Converter1 {
    type Output;

    fn convert(&self) -> Self::Output;
}

struct MyInt1;

impl Converter1 for MyInt1 {
    type Output = i32;

    fn convert(&self) -> Self::Output {
        42
    }
}

// 02

pub trait Converter2<T> {
    fn convert(&self) -> T;
}

struct MyInt2;

impl Converter2<i32> for MyInt2 {
    fn convert(&self) -> i32 {
        42
    }
}

fn main() {
    let my_int1 = MyInt1{};
    let output = my_int1.convert();
    println!("output is {}", output);

    let my_int2 = MyInt2{};
    let output = my_int2.convert();
    println!("output is {}", output);
}
