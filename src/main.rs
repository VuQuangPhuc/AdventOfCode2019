mod day_01;
mod day_02;
mod day_03;
mod day_04;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let arg = args[1].clone();

    match arg.as_str() {
        "1" => {
            day_01::solve();
        }
        "2" => {
            day_02::solve();
        },
        "3" => {
            day_03::solve();
        },
        "4" => {
            day_04::solve();
        }
        _ => {}
    }
}
