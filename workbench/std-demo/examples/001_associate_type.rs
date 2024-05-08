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

/*
https://cloud.tencent.com/developer/article/1561740

可以看到，在 trait 中，带上泛型参数，也可以实现关联类型同样的工作。但是，它们之间有区别。
trait 中的泛型与关联类型，有如下区别：
如果 trait 中包含泛型参数，那么，可以对同一个目标类型，多次 impl 此 trait，每次提供不同的泛型参数。而关联类型方式只允许对目标类型实现一次。
如果 trait 中包含泛型参数，那么在具体方法调用的时候，必须加以类型标注以明确使用的是哪一个具体的实现。而关联类型方式具体调用时不需要标注类型（因为不存在模棱两可的情况）。
*/
