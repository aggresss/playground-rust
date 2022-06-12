#[derive(Debug)]
enum Alphabet {
    Alpha(i32, i64),
    Beta{ index: i32, value: i64},
}

fn main() {
    let gamma = Alphabet::Alpha(15, 16);
    let delta = Alphabet::Beta{index:7, value:8};

    if let Alphabet::Alpha(a, b) = gamma {
        println!("{}, {}", a, b);
    }

    if let Alphabet::Beta { index: a, value: b } = delta {
        println!("{}, {}", a, b);
    }

    println!("{:?}", gamma);
    println!("{:?}", delta);
}
