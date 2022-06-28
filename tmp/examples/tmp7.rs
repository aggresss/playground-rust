pub trait Alpha {
    fn rho(&self) -> i32;
}

pub trait Beta {
    fn sigma(&self) -> i32;
}

struct AlphaBeta {}

impl Alpha for AlphaBeta {
    fn rho(&self) -> i32 {
        1
    }
}

impl Beta for AlphaBeta {
    fn sigma(&self) -> i32 {
        2
    }
}

fn main() {
    let a = AlphaBeta {};
    println!("{}, {}", a.sigma(), a.rho());
}
