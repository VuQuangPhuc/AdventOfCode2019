use crate::_reader;
use std::fs::File;

pub fn solve() -> () {
    let file = File::open("./input/day02.csv").unwrap();
    let input = _reader::read_program(file);
    println!("First solution: {}", p_one(input.clone()));
    println!("Second solution: {:?}", p_two(input.clone()));
}

fn p_one(mut input: Vec<i32>) -> i32 {
    execute(init_memory(&mut input, 12, 2))
}

fn p_two(input: Vec<i32>) -> (i32, i32) {
    for i in 0..100 {
        for j in 0..100 {
            let mut base = input.clone();
            let mut memory = init_memory(&mut base, i, j);
            if execute(&mut memory) == 19690720 {
                return (i, j);
            }
        }
    }

    return (0, 0);
}

fn execute(input: &mut Vec<i32>) -> i32 {
    let mut ip = 0;
    loop {
        let opcode = input[ip];
        match opcode {
            99 => {
                return input[0];
            }
            1 => {
                let (t, lhs, rhs) = get_addresses(&input, ip);
                input[t] = input[lhs] + input[rhs];
            }
            2 => {
                let (t, lhs, rhs) = get_addresses(&input, ip);
                input[t] = input[lhs] * input[rhs];
            }
            _ => println!("FUCK"),
        }
        ip = ip + 4;
    }
}

fn get_addresses(input: &Vec<i32>, ip: usize) -> (usize, usize, usize) {
    (
        input[ip + 3] as usize,
        input[ip + 1] as usize,
        input[ip + 2] as usize,
    )
}

fn init_memory(input: &mut Vec<i32>, noun: i32, verb: i32) -> &mut Vec<i32> {
    input[1] = noun;
    input[2] = verb;
    input
}
