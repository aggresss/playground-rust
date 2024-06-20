use mylib;

mylib::make_answerx!();


macro_rules! make_answery {
    ($name:expr) => {
        println!("Hello, {}!", $name);
    };
}

fn main() {
    println!("{}", answer());
    make_answery!("alpha");
}
