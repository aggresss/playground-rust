pub mod bear;
mod fly;
mod runner;

pub use bear::*;
pub use fly::fly_bird as now_fly_brid;
pub use runner::dog_run;

const HIGH_GAIN_1: f64 = 1.11;
pub const HIGH_GAIN_2: f64 = 2.22;

pub fn bear_play() {
    bear::bear_play();
}
