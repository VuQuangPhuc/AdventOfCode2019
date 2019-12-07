mod _reader;
mod day_01;
mod day_02;

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
        }
        _ => {}
    }
}
