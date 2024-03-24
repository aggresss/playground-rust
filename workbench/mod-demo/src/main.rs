mod toy;
mod toy_conductor;

mod toy1 {
    include!("./toy_implements.rs");
}

#[path = "./toy_implements.rs"]
mod toy2;

fn main() {
    toy::dog_run();
    toy::now_fly_brid();
    toy::bear_eat();
    toy::bear_sleep();

    toy::bear::bear_eat();
    toy::bear::bear_sleep();

    toy::bear_play();

    toy1::run();
    toy2::run();

    toy_conductor::nested::run();
    toy_conductor::run();

    println!("{}", toy::HIGH_GAIN_2)
}
