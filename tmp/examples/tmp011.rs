#![allow(dead_code)]

use std::convert::From;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

fn main() {
    let num = Number::from(32); // 函数类似于String::from()（String 转成 &str）
    println!("My Number-num is = {:?}", num);

    let int_0 = 6;
    let num2: Number = int_0.into(); // 函数类似于to_string（&str 转成 String）
    println!("My Number-num2 is = {:?}", num2);
}
// My Number-num is = Number { value: 32 }
// My Number-num2 is = Number { value: 6 }
