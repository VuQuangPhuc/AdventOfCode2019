pub fn solve() -> () {
    let input = include_str!("../../input/day01.csv");
    let modules = parse_input(input);
    let mods = modules.iter().map(calc_mod_fuel);
    let fuel: i32 = mods.map(calc_fuel_fuel).sum();
    println!("{}", fuel)
}

fn parse_input(input: &str) -> Vec<i32> {
    input
        .lines()
        .map(|line| line.parse().unwrap())
        .collect()
}

fn calc_mod_fuel(m: &i32) -> i32 {
    (*m / 3) - 2
}

fn calc_fuel_fuel(weight: i32) -> i32 {
    if weight <= 0 {
        return 0;
    }

    let add_fuel = std::cmp::max(calc_mod_fuel(&weight), 0);
    return weight + calc_fuel_fuel(add_fuel);
}
