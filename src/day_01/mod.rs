use crate::_reader;
use std::fs::File;

pub fn solve() -> () {
    let file = File::open("./input/day01.csv").unwrap();
    let input = _reader::read_lines(file);
    let mods = input.iter().map(calc_mod_fuel);
    let fuel: i32 = mods.map(calc_fuel_fuel).sum();
    println!("{}", fuel)
}

fn calc_mod_fuel(m: &i32) -> i32 {
    (*m / 3) - 2
}

fn calc_fuel_fuel(weight: i32) -> i32 {
    if weight <= 0 {
        return 0
    }

    let add_fuel = std::cmp::max(calc_mod_fuel(&weight), 0);
    return weight + calc_fuel_fuel(add_fuel)
}
